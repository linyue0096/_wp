function fetchData(id, callback) {
  const data = { id: id, status: "success" };
  callback(null, data);
}

fetchData(123, (err, result) => {
  if (err) return console.log("發生錯誤:", err);
  console.log("取得資料:", result); 
});