function A(num1, num2, B) 
{
  return B(num1, num2);
}
let resultAdd = A(10, 5, function(a, b) 
{
  return a + b;
});
let resultSub = A(10, 5, function(a, b) 
{
  return a - b;
});
console.log(resultAdd); 
console.log(resultSub); 