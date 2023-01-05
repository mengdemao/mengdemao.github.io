// 1. ES5定义对象

// 1.1 对象初始化器
var student1 = {
	name: "hello",
	age: 18,
	printStudent : function() {
		console.log('name : ' + this.name);
		console.log('age : ' + this.age);
	}
};

// 1.2 构造函数
function Student(name, age) {
	this.name = name;
	this.age = age;
	this.printStudent = function() {
		console.log('name : ' + this.name);
		console.log('age : ' + this.age);
	}
}
var student2 = new Student("world", 19);

// Object创建对象

var student3  = new Object();

// 添加成员

student3.name = "Hello";
student3.age  = 18;
student3.printStudent = function() {
	console.log('name : ' + this.name);
	console.log('age : ' + this.age);
}

// 修改属性
student3.age = 19;
student3.name = "world";

// 修改方法
student3.printStudent = function() {
	console.log('new name : ' + this.name);
	console.log('new age : ' + this.age);
}

// 执行函数调用
student1.printStudent();
student2.printStudent();
student3.printStudent();

// 判断类型
console.log('type of student1 is ' + toString.call(student1));
console.log('type of student2 is ' + toString.call(student2));
console.log('type of student3 is ' + toString.call(student3));

// 删除方法
delete(student3.name);
student3.printStudent();
