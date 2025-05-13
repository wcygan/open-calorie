use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FoodInfo {
    pub name: String,
    pub calories: u32,
    pub fat: u32,
    pub protein: u32,
    pub carbs: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_foodinfo_roundtrip() {
        let food = FoodInfo {
            name: "Apple".to_string(),
            calories: 95,
            fat: 0,
            protein: 0,
            carbs: 25,
        };

        // Serialize to JSON string
        let json = serde_json::to_string(&food).unwrap();

        // Deserialize back to struct
        let deserialized: FoodInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(food.name, deserialized.name);
        assert_eq!(food.calories, deserialized.calories);
        assert_eq!(food.fat, deserialized.fat);
        assert_eq!(food.protein, deserialized.protein);
        assert_eq!(food.carbs, deserialized.carbs);
    }
    
    #[test]
        fn test_foodinfo_from_json_string() {
            let json = r#"{
                "name": "Banana",
                "calories": 105,
                "fat": 0,
                "protein": 1,
                "carbs": 27
            }"#;
    
            let food: FoodInfo = serde_json::from_str(json).unwrap();
            assert_eq!(food.name, "Banana");
            assert_eq!(food.calories, 105);
            assert_eq!(food.fat, 0);
            assert_eq!(food.protein, 1);
            assert_eq!(food.carbs, 27);
    
            let serialized = serde_json::to_string(&food).unwrap();
            let deserialized: FoodInfo = serde_json::from_str(&serialized).unwrap();
            assert_eq!(food, deserialized);
        }
}
