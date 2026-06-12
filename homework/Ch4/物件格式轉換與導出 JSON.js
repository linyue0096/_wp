function exportProductData(names, prices) {
  let products = [];
  for (let i = 0; i < names.length; i++) {
    products.push({
      id: i + 1,
      name: names[i],
      price: prices[i]
    });
  }
  // 將 JavaScript 陣列轉為 JSON 字串
  return JSON.stringify(products);
}

console.log(exportProductData(["Pen", "Notebook"], [15, 50])); 
// 輸出: '[{"id":1,"name":"Pen","price":15},{"id":2,"name":"Notebook","price":50}]'