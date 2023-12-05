mod message;
mod modal_type;

pub use self::{message::Message, modal_type::ModalType};

/// Модальный диалог создания чего-либо
pub struct CreationModal {
    /// Значение, записанное пользователем в поле ввода модального окна
    pub input_value: String,
    /// Тип модального диалога создания объекта
    pub modal_type: ModalType,
}

impl CreationModal {
    const DEFAULT_INPUT_VALUE_CAPACITY: usize = 32;

    pub fn new(modal_type: ModalType) -> Self {
        Self {
            input_value: String::with_capacity(Self::DEFAULT_INPUT_VALUE_CAPACITY),
            modal_type,
        }
    }

    pub fn clear(&mut self) {
        self.input_value.clear();
    }
}
