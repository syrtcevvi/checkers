use std::{cell::RefCell, rc::Rc};

use iced::{
    event::{self, Event},
    executor,
    keyboard::{self, KeyCode, Modifiers},
    subscription, theme, time,
    widget::{
        button, checkbox, column, container, pick_list, row, slider, text, text_input, Row, Text,
    },
    window, Alignment, Application, Color, Command, Element, Length, Renderer, Subscription, Theme,
};

use iced_aw::{helpers::menu_tree, menu_bar, menu_tree, modal, Card, MenuTree};

use crate::application::{
    button_style::ButtonStyle,
    enums::{Message, Side},
    io::persist_vcs_in_file,
    structs::{
        Board, BoardMessage, CreationModal, CreationModalMessage, GameData, ModalType, Vcs,
        VcsMessage,
    },
};

pub struct Checkers {
    /// Игральная доска
    board: Board,
    /// Данные о состоянии игры
    game_data: Rc<RefCell<GameData>>,
    /// Система контроля версий
    vcs: Vcs,
    /// Модальное окно создания
    creation_modal: Option<CreationModal>,
}

impl Checkers {
    const MODAL_WINDOW_WIDTH: f32 = 420.0;

    fn vcs_controls<'a>(&self) -> Element<'a, Message> {
        row![
            column![
                Text::new("Выбор ветки"),
                pick_list(
                    self.vcs.branch_names(),
                    Some(self.vcs.current_branch_name()),
                    |branch_name: String| Message::Vcs(VcsMessage::SwitchToBranch(branch_name))
                )
            ],
            column![
                Text::new("Выбор снимка"),
                pick_list(
                    self.vcs.commit_headers(),
                    self.vcs.current_commit_header(),
                    |commit_header: String| {
                        let id: isize = commit_header.split_once("-").unwrap().0.parse().unwrap();
                        Message::Vcs(VcsMessage::SwitchToCommit(id))
                    }
                )
            ],
        ]
        .padding(8)
        .spacing(20)
        .into()
    }

    /// Создаёт кнопку для меню верхнего уровня
    fn menubar_button<'a>(
        content: impl Into<Element<'a, Message, Renderer>>,
    ) -> button::Button<'a, Message, Renderer> {
        button(content)
            .padding([4, 8])
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
            .on_press(Message::None)
    }

    /// Создаёт кнопку для вложенного меню
    fn menu_button<'a>(
        content: impl Into<Element<'a, Message, Renderer>>,
        message: Option<Message>,
    ) -> button::Button<'a, Message, Renderer> {
        let btn = button(content)
            .padding([4, 8])
            .width(200)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})));

        if let Some(message) = message {
            btn.on_press(message)
        } else {
            btn
        }
    }

    /// Создаёт элемент внутреннего меню
    fn menu_item<'a>(
        label: &'static str,
        message: Option<Message>,
    ) -> MenuTree<'a, Message, Renderer> {
        menu_tree!(Self::menu_button(label, message))
    }

    /// Создаёт элемент меню версионного контроля
    fn vcs_menu<'a>(&self) -> MenuTree<'a, Message, Renderer> {
        menu_tree(
            Self::menubar_button("СКВ"),
            vec![
                // TODO Проверить условие того, что пользователю дозволено создать новый снимок
                Self::menu_item(
                    "Создать снимок",
                    if self.vcs.is_commit_creation_allowed() {
                        Some(Message::CreationModal(CreationModalMessage::Open(
                            ModalType::CommitCreation,
                        )))
                    } else {
                        None
                    },
                ),
                Self::menu_item(
                    "Создать ветку",
                    Some(Message::CreationModal(CreationModalMessage::Open(
                        ModalType::BranchCreation,
                    ))),
                ),
            ],
        )
    }

    fn creation_modal_overlay<'a>(
        &self,
        label: &'static str,
        input_placeholder: &'static str,
    ) -> Element<'a, Message, Renderer> {
        let creation_modal = self.creation_modal.as_ref().unwrap();

        Card::<_, Renderer>::new(
            label,
            row![text_input(input_placeholder, &creation_modal.input_value)
                .on_input(
                    |value| Message::CreationModal(CreationModalMessage::TextInputChanged(value))
                )
                .on_submit(Message::CreationModal(CreationModalMessage::Finished(
                    creation_modal.input_value.clone()
                )))],
        )
        .max_width(Self::MODAL_WINDOW_WIDTH)
        .into()
    }

    fn commit_creation_modal_overlay<'a>(&self) -> Element<'a, Message, Renderer> {
        self.creation_modal_overlay("Создание снимка", "Введите сообщение")
    }

    fn branch_creation_modal_overlay<'a>(&self) -> Element<'a, Message, Renderer> {
        self.creation_modal_overlay("Создание ветки", "Введите название")
    }
}

impl Application for Checkers {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let game_data = Rc::new(RefCell::new(GameData::default()));
        // TODO проверяем, возможно ли восстановить состояние СКВ из файла

        (
            Self {
                board: Board::new(game_data.clone()),
                game_data: game_data,
                vcs: Vcs::default(),
                creation_modal: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Шашки с системой контроля версий")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Board(board_message) => {
                match board_message {
                    BoardMessage::MovePiece { from, to, side } => {
                        let mut game_data = self.game_data.borrow_mut();
                        game_data.move_piece(side, from, to);
                        if game_data.is_turning_to_king_condition_satisfied(side, to) {
                            game_data.turn_man_to_king(side, to);
                            // После превращения шашки в дамку ход продолжается
                        } else {
                            game_data.pass_the_move();
                        }
                    }
                    BoardMessage::TakePieces {
                        from,
                        to,
                        side,
                        taken_pieces_positions,
                    } => {
                        let mut game_data = self.game_data.borrow_mut();
                        // Убираем все фишки на пути, так как они были "съедены" во время взятия
                        game_data.remove_pieces(&taken_pieces_positions, side.opposite());
                        game_data.move_piece(side, from, to);
                        if game_data.is_turning_to_king_condition_satisfied(side, to) {
                            game_data.turn_man_to_king(side, to);
                            // После превращения шашки в дамку ход продолжается
                        } else {
                            game_data.pass_the_move();
                        }
                        if game_data.is_game_ended() {
                            return Command::perform(std::future::ready(()), |_| {
                                Message::Board(BoardMessage::Restart)
                            });
                        }
                    }
                    BoardMessage::Restart => {
                        self.game_data.replace(GameData::default());
                    }
                }
                self.board.update();
            }
            Message::Vcs(vcs_message) => match vcs_message {
                VcsMessage::SwitchToBranch(branch_name) => {
                    if let Some(game_data) = self.vcs.switch_to_branch(branch_name) {
                        self.game_data.replace(game_data);
                        // Перерисовываем доску
                        self.board.update();
                    }
                }
                VcsMessage::SwitchToCommit(commit_id) => {
                    self.game_data.replace(self.vcs.switch_to_commit(commit_id));
                    self.board.update();
                }
            },
            Message::CreationModal(creation_modal_message) => match creation_modal_message {
                CreationModalMessage::Open(modal_type) => {
                    if let None = self.creation_modal {
                        self.creation_modal = Some(CreationModal::new(modal_type));
                    }
                }
                CreationModalMessage::TextInputChanged(value) => {
                    if let Some(creation_modal) = &mut self.creation_modal {
                        creation_modal.input_value = value;
                    }
                }
                CreationModalMessage::Close => {
                    self.creation_modal = None;
                }
                CreationModalMessage::Finished(value) => {
                    match self.creation_modal.as_ref().unwrap().modal_type {
                        ModalType::BranchCreation => {
                            self.vcs.create_branch(value);
                        }
                        ModalType::CommitCreation => {
                            self.vcs
                                .create_commit(value, self.game_data.borrow().clone());
                        }
                    }
                    self.creation_modal = None;
                }
            },
            Message::EventOccured(event) => {
                if let Event::Window(event) = event {
                    if let window::Event::CloseRequested = event {
                        match persist_vcs_in_file(&self.vcs) {
                            Ok(_) => println!("СКВ успешно была записана в файл!"),
                            Err(err) => {
                                println!("Возникла ошибка во время записи СКВ в файл: {err}")
                            }
                        };
                        return window::close();
                    }
                }
            }
            Message::None => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccured)
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let underlay = container(column![
            menu_bar!(self.vcs_menu()),
            self.board.view().map(Message::Board),
            self.vcs_controls()
        ]);

        let overlay =
            self.creation_modal
                .as_ref()
                .map(|creation_modal| match creation_modal.modal_type {
                    ModalType::BranchCreation => self.branch_creation_modal_overlay(),
                    ModalType::CommitCreation => self.commit_creation_modal_overlay(),
                });

        modal(underlay, overlay)
            .on_esc(Message::CreationModal(CreationModalMessage::Close))
            .backdrop(Message::CreationModal(CreationModalMessage::Close))
            .into()
    }
}
