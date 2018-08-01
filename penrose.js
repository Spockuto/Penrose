function createTriangle(colour , a , b , c){
	var triangleObject = new Object();
	triangleObject["colour"] = colour;
	triangleObject["a"] = a;
	triangleObject["b"] = b;
	triangleObject["c"] = c;
	return triangleObject;
}

function GenerateTypeOne(count) {
	triangleSet = [];
	for(var i = 0 ;  i < 10 ; i++){
		var r = radius;
		var b = math.complex(r * Math.cos((2*i - 1) * math.pi / 10), r *  Math.sin((2*i - 1) * math.pi / 10));
		var c = math.complex(r * Math.cos((2*i + 1) * math.pi / 10), r * Math.sin((2*i + 1) * math.pi / 10));
		if(i % 2 == 0){
			[b , c] = [c, b];
		}
		triangleSet.push(createTriangle(0, math.complex(), b , c));
	}

	for(var i = 0;  i < count ; i ++) { 
		var result = [];
		triangleSet.forEach(function(triangle){
			if(triangle.colour == 0){
				var p = math.add(triangle.a, math.subtract(triangle.b, triangle.a).div(goldenRatio));
				result.push(createTriangle(0, triangle.c, p, triangle.b));
				result.push(createTriangle(1, p , triangle.c , triangle.a ));
			}
			else {
				var q = math.add(triangle.b, math.subtract(triangle.a, triangle.b).div(goldenRatio));
				var r = math.add(triangle.b, math.subtract(triangle.c, triangle.b).div(goldenRatio));
				result.push(createTriangle(1, r, triangle.c, triangle.a));
				result.push(createTriangle(1, q , r, triangle.b));
				result.push(createTriangle(0, r , q, triangle.a ));
			}
		});
		triangleSet = result;
	}
}


function GenerateTypeTwo(count) {
	triangleSet = [];
	for(var i = 0 ;  i < 10 ; i++){
		var r = radius;
		var b = math.complex(r * Math.cos((2*i - 1) * math.pi / 10), r *  Math.sin((2*i - 1) * math.pi / 10));
		var c = math.complex(r * Math.cos((2*i + 1) * math.pi / 10), r * Math.sin((2*i + 1) * math.pi / 10));
		if(i % 2 == 0){
			[b , c] = [c, b];
		}
		triangleSet.push(createTriangle(0, b , math.complex(),  c));
	}

	for(var i = 0;  i < count ; i ++) { 
		var result = [];
		triangleSet.forEach(function(triangle){
			if(triangle.colour == 0){
				var q = math.add(triangle.a, math.subtract(triangle.b, triangle.a).div(goldenRatio));
				var r = math.add(triangle.b, math.subtract(triangle.c, triangle.b).div(goldenRatio));
				result.push(createTriangle(1, r , q, triangle.b));
				result.push(createTriangle(0, q, triangle.a , r));
				result.push(createTriangle(0 ,triangle.c, triangle.a , r));
			}
			else {
				var p = math.add(triangle.c, math.subtract(triangle.a, triangle.c).div(goldenRatio));
				result.push(createTriangle(0, triangle.b, p, triangle.a));
				result.push(createTriangle(1, p , triangle.c , triangle.b ));				
			}
		});
		triangleSet = result;
	}
}


function selectFractal(i){
	if (i == 1){
		type = 1;
	}
	if (i == 2){
		type = 2;
	}
}

function resizeCanvas(){
	context.canvas.height = document.getElementById("div2").clientHeight;
	context.canvas.width = document.getElementById("div2").clientWidth;
	radius = canvas.height * 0.5;
	var transX = canvas.width * 0.5,
    transY = canvas.height * 0.5;
	context.translate(transX, transY);
	context.clearRect(0, 0, canvas.width, canvas.height);
}

function Canvas(){
	resizeCanvas();

	iterator = document.getElementById("iterator").value;

	if(iterator ==  ""){
		iterator = 7;
	}

	if(document.getElementById("color1").value != "")
		color1 = document.getElementById("color1").value;

	if(document.getElementById("color2").value != "")
		color2 = document.getElementById("color2").value;
	
	if(type == 1){
		GenerateTypeOne(parseInt(iterator));
	}

	if(type == 2){
		GenerateTypeTwo(parseInt(iterator));
	}

	triangleSet.forEach(function(triangle){
		context.beginPath();
		context.moveTo(triangle.c.re, triangle.c.im);
		context.lineTo(triangle.a.re, triangle.a.im);
		context.lineTo(triangle.b.re, triangle.b.im);
		if (triangle.colour == 0 )
			context.fillStyle = color1;
		else 
			context.fillStyle = color2;
		context.fill();
		context.stroke();	
	});
}

var goldenRatio = ( 1 + Math.sqrt(5) )  / 2;
var triangleSet = [] ;
var canvas = document.getElementById("penroseCanvas");
var context = canvas.getContext("2d");
var radius = 400;
var type = 0;
var iteration = 0;
var color1 = "#40C4FF";
var color2 = "#64FFDA";
GenerateTypeTwo(7);
resizeCanvas();