var text = '{ "student" : [' +
'{ "name":"hello" , "age": 18 },' +
'{ "name":"world" , "age": 18 }]}';

var jsonObj = JSON.parse(text);
var jsonTxt = JSON.stringify(jsonObj);

console.log(jsonObj);
console.log(jsonTxt);
