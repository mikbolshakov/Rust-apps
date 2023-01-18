#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper {

    /// Определяем место хранения вашего контракта.
    /// Добавляем новые поля в приведенную ниже структуру по порядку,
    /// чтобы добавить новые статические поля хранения в ваш контракт.
    #[ink(storage)]
    pub struct Flipper {
        /// Сохраняет одно значение `bool` в хранилище.
        value: bool,
    }

    impl Flipper {
        /// Конструктор, который инициализирует значение bool заданным значением init_value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Конструктор, который инициализирует значение `bool` значением `false`.
        /// Конструкторы могут делегировать полномочия другим конструкторам.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Сообщение, которое можно вызвать для экземпляров контрактов.
        /// Оно меняет значение сохраненного `bool` от `true` в `false` и наоборот.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Просто возвращает текущее значение нашего `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Юнит тесты в Rust обычно определяются внутри такого `#[cfg(test)]`,
    /// модуль и тестовые функции отмечены атрибутом `#[test]`.
    /// Приведенный ниже код технически является обычным кодом Rust.
    #[cfg(test)]
    mod tests {
        /// Импортирует все определения из внешней области видимости, чтобы мы могли использовать их здесь.
        use super::*;

        /// Мы проверяем, выполняет ли конструктор по умолчанию свою работу.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// Мы тестируем простой вариант использования нашего контракта.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
