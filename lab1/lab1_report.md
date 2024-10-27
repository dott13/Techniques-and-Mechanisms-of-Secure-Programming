# SOLID Principles

## By Popov Tudor

## Objectives

- Implement 2 letters from SOLID.
- Learn and understand about the importance and usage of SOLID Principles.

## Used Principles

- S(SRP): Single Responsability Principle
- C(OCP): Open/Closed Principle

## Implementation

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

### Open/Closed Principle (OCP)

The Open/Closed Principle states that software entities (classes, modules, functions) should be open for extension but closed for modification. This means that we can add new functionality without altering existing code. In our implementation:

- The `Discount` trait allows adding new types of discounts by implementing apply_discount without changing existing structures.
- The `PercentageDiscount` struct applies a percentage-based discount, and new discount types can be added by implementing Discount.

```rust
trait Discount {
    fn apply_discount(&self, total: f64) -> f64;
}

// PercentageDiscount applies a percentage-based discount
struct PercentageDiscount {
    percent: f64,
}

impl Discount for PercentageDiscount {
    fn apply_discount(&self, total: f64) -> f64 {
        total - (total * self.percent / 100.0)
    }
}
```

## Conclusion

In this project, we successfully implemented the Single Responsibility and Open/Closed Principles from SOLID, achieving modular and extendable code. By ensuring that each struct, trait, and function has a focused responsibility, we created a well-structured foundation where functionality can be added with minimal changes to the existing code. The Product and Cart structs illustrate SRP by handling distinct responsibilities, while the Discount trait demonstrates OCP, allowing for new discount types without altering existing logic. These principles enhance maintainability, making future development more manageable and efficient.
