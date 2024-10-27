### Single Responsibility Principle (SRP)

The Single Responsibility Principle states that a class should have only one reason to change, meaning it should have only one responsibility. In our implementation:

- The `Product` struct is responsible solely for managing product details, such as `id`, `name`, and `price`.
- The `Cart` struct is responsible for managing the products added to the cart and calculating the total price. Each struct is focused on a single aspect of the functionality, making the code easier to maintain and extend.

Examples:

```rust
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
```

Example code for the Cart struct that just add totals and displays the Cart itself:

```rust
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

    fn total(&self) -> f64 {
        let total: f64 = self.products.iter().map(|p| p.price).sum();
        if let Some(discount) = &self.discount {
            discount.apply_discount(total)
        } else {
            total
        }
    }
```

## Conclusion

In this project, we have successfully implemented the Single Responsibility Principle and the Open/Closed Principle, laying the groundwork for building robust and maintainable software. By adhering to these principles, we can easily extend our application in the future with new features while keeping the existing code clean and manageable.
