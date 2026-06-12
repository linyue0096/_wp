function calculateTotal(jsonString) {
  // 將 JSON 字串轉換為 JavaScript 陣列/物件
  const cart = JSON.parse(jsonString); 
  let total = 0;
  
  for (let i = 0; i < cart.length; i++) {
    total += cart[i].price * cart[i].quantity;
  }
  return total;
}

const jsonInput = '[{"item":"Apple","price":20,"quantity":3},{"item":"Banana","price":10,"quantity":5}]';
console.log(calculateTotal(jsonInput)); // 輸出: 110