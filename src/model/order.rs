#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderItem {
    pub product: String,
    pub quantity: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub items: Vec<OrderItem>,
    pub user: Option<String>,
}

impl Order {
    pub fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }
}
