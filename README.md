# Credit Card Limit Tracker App

This is a simple Rust project built using the Dioxus framework, designed to help track your credit card limit and manage payments between two bank accounts.

## Purpose

The app helps you track your credit card usage by showing how much you owe and how much you need to transfer from your salary account to your credit card payment account. It keeps a record of your credit card transactions and updates your available limit accordingly.

### Example Scenario

1. **Setting a limit:**
   You set a credit card limit, for example, ₹20,000.

2. **Making a purchase:**
   You buy an item worth ₹2,000. Your credit card debt now becomes ₹2,000, and your available limit decreases to ₹18,000.

3. **Transferring money:**
   You transfer ₹2,000 from your salary account (Bank A) to your credit card payment account (Bank B).

4. **Buying again:**
   You buy an item worth ₹3,000. Now your total debt is ₹5,000 (₹2,000 from the previous purchase and ₹3,000 from this one). The available credit limit is now ₹15,000.

5. **Tracking your payments:**
   On the app, you can input the total amount owed to your credit card (₹5,000) and the app will automatically tell you how much money you need to transfer from Bank A to Bank B. For this example, it will tell you to transfer ₹3,000.

---

## Features

- **Track Credit Card Limit:** See how much debt you owe and your remaining limit.
- **Transaction History:** View your past credit card transactions.
- **Bank Transfers:** Input the amount you owe, and the app tells you how much to transfer from your salary account to the credit card payment account.
- **Responsive UI:** Built using the Dioxus framework, providing a modern and responsive user interface.

---

## How to Run the Project

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/credit-card-limit-tracker.git
   cd credit-card-limit-tracker

2. Run it
```dx serve --platform web```

3. Tech Stack
# Rust:
Programming language used for backend logic.
# Dioxus:
A Rust framework for building fast, reactive web apps with an ergonomic API.
Async/Await: For handling asynchronous operations, such as fetching transactions and saving data.

# Contributing
Feel free to fork the repository, make changes, and create a pull request. Contributions are welcome!

# License
This project is licensed under the MIT License - see the LICENSE file for details.

# Contact
For any issues or feature requests, feel free to open an issue or reach out to me at [anjal.afghan93@gmail.com]
