#![cfg_attr(not(feature = "std"),no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod storage {
    #[ink(storage)]
    pub struct Storage {
        cid: String,
    }

    impl Storage {
        #[ink(constructor)]
        pub fn new(value: String) -> Self {
            Self{cid: value}
        }

        #[ink(message)] 
        pub fn get(&self) -> String {
            self.cid.to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let val = Storage::new("hello".to_string());
            assert_eq!(val.get(), "hello");
        }
    }
}
