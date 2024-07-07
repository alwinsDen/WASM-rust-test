import { chars } from "./chars-list";
const imp = import("./pkg");
let mod;
let counters = [];

imp.then((wasm) => {
	mod = wasm;
	addCounter();
	let b = document.getElementById("add-counter");
	b.addEventListener("click", (ev) => addCounter());
});

function addCounter() {
	let ctr = mod.Counter.new(randomCharacter(), 0);
	counters.push(ctr);
	update();
}

function randomCharacter() {
	let idx = Math.floor(Math.random() * (chars.length - 1));
	let ret = chars.splice(idx, 1)[0];
	return ret;
}

// here is the update function
function update() {
	let container = document.getElementById("container");
	if (!container) throw new Error("Unable to find #container in dom");
	while (container.hasChildNodes()) {
		if (container.lastChild.id == "add-counter") break;
		container.removeChild(container.lastChild);
	}
	for (var i = 0; i < counters.length; i++) {
		let counter = counters[i];
		container.appendChild(
			newCounter(counter.key(), counter.count(), (ev) => {
				counter.increment();
				update();
			}),
		);
	}
}

//the new counter function
function newCounter(key, value, cb) {
	let container = document.createElement("div");
	container.setAttribute("class", "counter");
	let title = document.createElement("h1");
	title.appendChild(document.createTextNode("Counter " + key));
	container.appendChild(title);
	container.appendChild(newField("Count", value));
	let plus = document.createElement("button");
	plus.setAttribute("type", "button");
	plus.setAttribute("class", "plus-button");
	plus.appendChild(document.createTextNode("+"));
	plus.addEventListener("click", cb);
	container.appendChild(plus);
	return container;
}

function newField(key, value) {
	let ret = document.createElement("div");
	ret.setAttribute("class", "field");
	let name = document.createElement("span");
	name.setAttribute("class", "name");
	name.appendChild(document.createTextNode(key));
	ret.appendChild(name);
	let val = document.createElement("span");
	val.setAttribute("class", "value");
	val.appendChild(document.createTextNode(value));
	ret.appendChild(val);
	return ret;
}
