function getAdults(people) {
  let adults = [];
  for (let i = 0; i < people.length; i++) {
    if (people[i].age >= 18) {
      adults.push(people[i]);
    }
  }
  return adults;
}

const users = [
  { name: "Tom", age: 15 },
  { name: "Jerry", age: 20 },
  { name: "Mickey", age: 35 }
];
console.log(getAdults(users)); 
// 輸出: [ { name: 'Jerry', age: 20 }, { name: 'Mickey', age: 35 } ]