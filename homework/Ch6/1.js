function mathTool(num1, num2, action) 
{
  return action(num1, num2);
}
let resultAdd = mathTool(10, 5, function(a, b) 
{
  return a + b;
});
let resultSub = mathTool(10, 5, function(a, b) 
{
  return a - b;
});
console.log(resultAdd); 
console.log(resultSub); 