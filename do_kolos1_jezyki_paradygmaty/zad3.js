class Product {
  constructor(name, price) {
    this.name = name;
    this.price = price;
  }
}

class ProductInCart {
  constructor(product, quantity) {
    this.product = product;
    this.quantity = quantity;
  }
}

function displayCart(cart) {
  console.log("zawartosc koszyka:");
  for (const item of cart) {
    console.log(`   ${item.product.name} * ${item.quantity} = ${(item.product.price * item.quantity).toFixed(2)}`);
  }
}

function calculateTotal(cart) {
  let total = 0;

  for (const item of cart) {
    total += item.product.price * item.quantity;
  }

  return total;
}

const potato = new Product("ziemniak", 0.30);
const milk = new Product("mleko", 3.79);
const pierogi = new Product("pierogi", 12.99);
const wedka_na_spinning = new Product("wedka na spining", 97.21);
const karma_dla_kota = new Product("karma dla kota", 17.20);
const piwo_tyskie = new Product("Piwo tyskie", 2.99) 

const cart = [
  new ProductInCart(potato, 4), 
  new ProductInCart(milk, 2), 
  new ProductInCart(pierogi, 1),  
  new ProductInCart(wedka_na_spinning, 2), 
  new ProductInCart(piwo_tyskie, 20), // calom skrzynke
];

const totalToPay = calculateTotal(cart);
displayCart(cart); 
console.log("trzeba zaplacic:", totalToPay.toFixed(2), "szylingow jamajskich");
