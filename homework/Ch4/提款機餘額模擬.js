function withdrawUntilEmpty(account, amount) {
  let count = 0;
  while (account.balance >= amount) {
    account.balance -= amount;
    count++;
  }
  console.log(`成功提款 ${count} 次，帳戶剩餘: ${account.balance}`);
  return account;
}

let myAccount = { owner: "Bob", balance: 1000 };
withdrawUntilEmpty(myAccount, 300); 
// 輸出: 成功提款 3 次，帳戶剩餘: 100