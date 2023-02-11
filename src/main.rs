use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

struct PriceUpdate {
    instrument_id: u32,
    price: f64
}

impl PriceUpdate {
    fn new(instrument_id: u32, price: f64) -> Self {
        PriceUpdate {
            instrument_id,
            price
        }
    }
}

#[derive(Debug)]
struct Order {
    instrument_id: u32,
    qty: u32,
    limit_price: f64
}

impl Order {
    fn new(instrument_id: u32, qty: u32, limit_price: f64) -> Self {
        Order {
            instrument_id,
            qty,
            limit_price
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
enum OrderError {
    #[error("Invalid price")]
    InvalidPrice,
    #[error("Instrument not found")]
    InstrumentNotFound
}

struct PriceBookRepository {
    price_books: HashMap<u32, f64>,
}

impl PriceBookRepository {
    fn new() -> Self {
        PriceBookRepository {
            price_books: HashMap::new(),
        }
    }
}

struct PriceUpdateEventHandler {
    price_book_repository: Rc<RefCell<PriceBookRepository>>
}

impl PriceUpdateEventHandler {
    fn new(price_book_repository: Rc<RefCell<PriceBookRepository>>) -> Self {
        PriceUpdateEventHandler {
            price_book_repository,
        }
    }

    fn handle_price_update(&self, update: &PriceUpdate) {
        self.price_book_repository.borrow_mut().price_books.insert(update.instrument_id, update.price);
    }
}

struct OrderEventHandler {
    price_book_repository: Rc<RefCell<PriceBookRepository>>
}

impl OrderEventHandler {
    fn new(price_book_repository: Rc<RefCell<PriceBookRepository>>) -> Self {
        OrderEventHandler {
            price_book_repository,
        }
    }

    fn handle_order(&self, order: &Order) -> Result<(), OrderError> {
        // look up the price book for the product in the order
        let price_book_repository = self.price_book_repository.borrow();
        let price = price_book_repository.price_books.get(&order.instrument_id);

        // validate the order price
        if let Some(price) = price {
            if order.limit_price != *price {
                return Err(OrderError::InvalidPrice);
            }
        } else {
            return Err(OrderError::InstrumentNotFound);
        }

        Ok(())
    }
}

fn main() {
    let price_book_repo = Rc::new(RefCell::new(PriceBookRepository::new()));
    let price_update_handler = PriceUpdateEventHandler::new(price_book_repo.clone());
    let order_handler = OrderEventHandler::new(price_book_repo.clone());

    //this normally would come as an event be routed to an event handler based on event type.
    //but I just do it directly to play with it first
    let price_update = PriceUpdate::new(1, 45.32);
    price_update_handler.handle_price_update(&price_update);
    let order = Order::new(1, 100, 45.32);
    handle_order_result( &order, order_handler.handle_order(&order));

    let order = Order::new(1, 100, 45.31);
    handle_order_result( &order, order_handler.handle_order(&order));

    let order = Order::new(2, 100, 45.31);
    handle_order_result( &order, order_handler.handle_order(&order));
}

fn handle_order_result(order: &Order, result: Result<(), OrderError>) {
    match result {
        Ok(_) => println!("Order succeeded: {:?}", order),
        Err(err) => println!("Order error: {} in:  {:?}", err.to_string(), order)
    }
}
