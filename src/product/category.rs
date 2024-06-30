#[derive(Debug, PartialEq)]
pub enum Category {
    Electronics,
    Clothing,
    Food,
    Cosmetics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category() {
        assert_eq!(Category::Electronics, Category::Electronics);
        assert_eq!(Category::Clothing, Category::Clothing);
        assert_eq!(Category::Food, Category::Food);
        assert_eq!(Category::Cosmetics, Category::Cosmetics);
    }
}
