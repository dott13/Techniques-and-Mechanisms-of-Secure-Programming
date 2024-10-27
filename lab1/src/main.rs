use std::io::{self, Write};
use colored::*;
use rand::Rng;

// SOLID principle S(Single Responsability Principle) It takes responsability only for the product creation
struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl Product {
    fn new(id: u32, name: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
        }
    }
}

// Discount trait for different discount types (SOLID PRINCIPLE O: this is open to be extended the discound can be in different values not only in percentage and etc)
trait Discount {
    fn apply_discount(&self, total: f64) -> f64;
}

// PercentageDiscount for percentage-based discounts
struct PercentageDiscount {
    percent: f64,
}

impl Discount for PercentageDiscount {
    fn apply_discount(&self, total: f64) -> f64 {
        total - (total * self.percent / 100.0)
    }
}

// SOLID principle S on this aswell like for the Product
struct Cart {
    products: Vec<Product>,
    discount: Option<Box<dyn Discount>>,
}

impl Cart {
    fn new() -> Self {
        Cart {
            products: Vec::new(),
            discount: None,
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn set_discount(&mut self, discount: Box<dyn Discount>) {
        self.discount = Some(discount);
    }

    fn total(&self) -> f64 {
        let total: f64 = self.products.iter().map(|p| p.price).sum();
        if let Some(discount) = &self.discount {
            discount.apply_discount(total)
        } else {
            total
        }
    }

    fn display_cart(&self) {
        if self.products.is_empty() {
            println!("Your cart is empty.");
        } else {
            println!("\nItems in your cart:");
            for product in &self.products {
                println!("- {}: ${:.2}", product.name, product.price);
            }
            println!("Total: ${:.2}", self.total());
        }
    }
}

// Trait for OCP principle it can be extended aswell
trait ColorfulText {
    fn to_colorful_string(&self) -> String;
}

impl ColorfulText for str {
    fn to_colorful_string(&self) -> String {
        let mut rng = rand::thread_rng();
        self.chars()
            .map(|c| {
                let color = match rng.gen_range(0..=5) {
                    0 => c.to_string().red(),
                    1 => c.to_string().green(),
                    2 => c.to_string().yellow(),
                    3 => c.to_string().blue(),
                    4 => c.to_string().magenta(),
                    _ => c.to_string().cyan(),
                };
                color.to_string()
            })
            .collect::<String>()
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let welcome_text = "Welcome to Your Shopping Cart!";
    println!("{}", welcome_text.to_colorful_string());

    let mut cart = Cart::new();
    let mut product_id = 1;

    loop {
        println!("\n1. Add a product to the cart");
        println!("2. Apply a discount(%)");
        println!("3. View cart");
        println!("4. Checkout and Exit");

        let choice = get_input("Enter choice(1-4): ");
        match choice.as_str() {
            "1" => {
                let name = get_input("Enter product name: ");
                let price: f64 = get_input("Enter product price: MDL")
                    .parse()
                    .expect("Please enter a valid number");

                let product = Product::new(product_id, &name, price);
                cart.add_product(product);
                product_id += 1;
                println!("Added '{}' to your cart.", name);
            }
            "2" => {
                let percent: f64 = get_input("Enter discount(%): ")
                    .parse()
                    .expect("Please enter a valid number");

                let discount = PercentageDiscount { percent };
                cart.set_discount(Box::new(discount));
                println!("Applied a {}% discount.", percent);
            }
            "3" => {
                cart.display_cart();
            }
            "4" => {
                println!("\nFinal buy:");
                cart.display_cart();
                println!("Thank you for shopping! Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
