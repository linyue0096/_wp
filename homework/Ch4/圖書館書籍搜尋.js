const library = [
  { title: "Harry Potter", author: "J.K. Rowling" },
  { title: "The Hobbit", author: "J.R.R. Tolkien" },
  { title: "Fantastic Beasts", author: "J.K. Rowling" }
];

function findBooksByAuthor(lib, authorName) {
  let foundTitles = [];
  for (let i = 0; i < lib.length; i++) {
    if (lib[i].author === authorName) {
      foundTitles.push(lib[i].title);
    }
  }
  return foundTitles;
}

console.log(findBooksByAuthor(library, "J.K. Rowling")); 
// 輸出: [ 'Harry Potter', 'Fantastic Beasts' ]