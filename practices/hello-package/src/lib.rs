mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();
    
    back_of_house::cook_order();

    String::from("yummy yummy!")
}


#[cfg(test)]
pub mod tests {
    use crate::front_of_house;

    #[test]
    pub fn test_eat_at_restaurant() {
        assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
        assert_eq!(crate::eat_at_restaurant(), "yummy yummy!");
    }
}