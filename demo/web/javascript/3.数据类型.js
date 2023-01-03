// 定义数据
var a = 'Hello';			// 字符串(String)		
var b = 10;					// 数字(Number)
var c = true;				// 布尔(Boolean)
var d = function() {		// 函数(Function)
	console.log("Hello");
};
var e = [1, 2, 3];			// 数组(Array)
var f = null;				// 空(Null)
var g = Symbol();			// Symbol
var h;						// 未定义(Undefined)
var i = Object();			// 对象(Object)

// 1. typeof检测类型
console.log("\r\n1. typeof检测类型")
console.log('type of a is ' + typeof(a));
console.log('type of b is ' + typeof(b));
console.log('type of c is ' + typeof(c));
console.log('type of d is ' + typeof(d));
console.log('type of e is ' + typeof(e));
console.log('type of f is ' + typeof(f));
console.log('type of g is ' + typeof(g));
console.log('type of h is ' + typeof(h));
console.log('type of i is ' + typeof(i));

// 2. constructor返回实例的构造函数
console.log("\r\n2. constructor返回实例的构造函数")
console.log(a.constructor == String);
console.log(a.constructor == Number);

console.log(e.constructor == Object);
console.log(e.constructor == Array);

// 3. instanceof 原型查找
console.log("\r\n3. instanceof 原型查找")
console.log(a instanceof String);
console.log(a instanceof Number);

console.log(e instanceof Object);
console.log(e instanceof Array);

// 4. Object.prototype.toString
console.log("\r\n4. Object.prototype.toString")
console.log('type of a is ' + toString.call(a));
console.log('type of b is ' + toString.call(b));
console.log('type of c is ' + toString.call(c));
console.log('type of d is ' + toString.call(d));
console.log('type of e is ' + toString.call(e));
console.log('type of f is ' + toString.call(f));
console.log('type of g is ' + toString.call(g));
console.log('type of h is ' + toString.call(h));
console.log('type of i is ' + toString.call(i));
