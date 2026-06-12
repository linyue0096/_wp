function checkAdmin(role, callback) {
  if (role !== "admin") {
    callback("Access Denied");
  } else {
    callback(null, "Welcome");
  }
}

checkAdmin("user", (err, message) => {
  if (err) return console.log("測試 1 錯誤攔截:", err);
  console.log("測試 1 成功:", message);
});

checkAdmin("admin", (err, message) => {
  if (err) return console.log("測試 2 錯誤攔截:", err);
  console.log("測試 2 成功:", message);
});