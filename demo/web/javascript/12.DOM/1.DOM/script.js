// 文档标题
console.log("文档标题", document.title);
document.title = "Hello Dom";
console.log("文档标题", document.title);

// 加载脚本
function loadScript(url) {
	let script = document.createElement("script");
	script.src = url;
	document.body.appendChild(script);
}

// 加载CSS
function loadStyles(url){
	let link = document.createElement("link");
	link.rel = "stylesheet";
	link.type = "text/css";
	link.href = url;
	let head = document.getElementsByTagName("head")[0];
	head.appendChild(link);
}
