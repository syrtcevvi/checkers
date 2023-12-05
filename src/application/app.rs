use std::{cell::RefCell, rc::Rc};

use iced::{
    event::Event,
    executor,
    keyboard::{self, KeyCode, Modifiers},
    subscription, theme, time,
    widget::{
        button, checkbox, column, container, pick_list, row, slider, text, text_input, Row, Text,
    },
    Alignment, Application, Color, Command, Element, Length, Renderer, Subscription, Theme,
};

use iced_aw::{helpers::menu_tree, menu_bar, menu_tree, modal, Card, MenuTree};

use crate::application::{
    button_style::ButtonStyle,
    enums::{Message, Side},
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
        use crate::application::structs::VcsMessage;
        row![
            column![
                Text::new("Выбор ветки"),
                pick_list(
                    self.vcs.branch_names(),
                    Some(self.vcs.current_branch_name()),
                    |branch_name| Message::Vcs(VcsMessage::SelectBranch(branch_name))
                )
            ],
            // column![
            //     Text::new("Выбор снимка")
            // ],
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
        message: Message,
    ) -> button::Button<'a, Message, Renderer> {
        button(content)
            .padding([4, 8])
            .width(200)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
            .on_press(message)
    }

    /// Создаёт элемент внутреннего меню
    fn menu_item<'a>(label: &'static str, message: Message) -> MenuTree<'a, Message, Renderer> {
        menu_tree!(Self::menu_button(label, message))
    }

    /// Создаёт элемент меню версионного контроля
    fn vcs_menu<'a>(&self) -> MenuTree<'a, Message, Renderer> {
        menu_tree(
            Self::menubar_button("СКВ"),
            vec![
                Self::menu_item(
                    "Создать снимок",
                    Message::CreationModal(CreationModalMessage::Open(ModalType::CommitCreation)),
                ),
                Self::menu_item(
                    "Создать ветку",
                    Message::CreationModal(CreationModalMessage::Open(ModalType::BranchCreation)),
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
                        // TODO Проверка условия превращения шашки в дамку
                        game_data.pass_the_move();
                    }
                }
                self.board.update();
            }
            Message::Vcs(vcs_message) => match vcs_message {
                VcsMessage::SelectBranch(branch) => {
                    println!("Пользователь выбрал ветку: {}", branch);
                    // TODO
                }
                VcsMessage::SelectCommit(commit) => {
                    println!("Пользователь выбрал коммит: {}", commit);
                    // TODO
                }
            },
            Message::CreationModal(creation_modal_message) => match creation_modal_message {
                // TODO
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
                CreationModalMessage::Finished(value) => {}
                _ => {}
            },
            Message::None => {}
        }
        Command::none()
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
