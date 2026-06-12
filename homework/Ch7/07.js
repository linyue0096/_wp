function fakeGet(sql, params, callback) {
  callback(null, { title: "Fake Title" });
}

fakeGet("SELECT * FROM posts WHERE id = ?", [1], (err, row) => {
  if (err) return console.log("資料庫錯誤:", err);
  console.log("文章標題:", row.title); 
});