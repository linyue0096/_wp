function createUser(name, age, email) {
  let user = {
    userName: name,
    userAge: age,
    userEmail: email
  };
  return user;
}

console.log(createUser("Alice", 25, "alice@example.com")); 
// 輸出: { userName: 'Alice', userAge: 25, userEmail: 'alice@example.com' }