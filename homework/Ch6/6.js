function myFilter(arr, callback) 
{
  let result = [];
  for (let i = 0; i < arr.length; i++) 
    {
    if (callback(arr[i])) {
      result.push(arr[i]);
    }
  }
  return result;
}
let data = [1, 5, 8, 12];
let filtered = myFilter(data, function(item) 
{
  return item > 7;
});
console.log(filtered);