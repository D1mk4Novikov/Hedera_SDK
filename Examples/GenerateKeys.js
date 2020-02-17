// подключение фреймворка
const Excalibur = require("../JavaScript/Excalibur.js");
// создание экземпляра класса
let hashgraph = new Excalibur("_nodeAddress_", "_nodeAccount_", true);
// вызов функции генерации ключей
hashgraph.generateKeys()
	.then(result => {
		// Вывод сгенеренных ключей
		console.log("Private key: " + result.privateKey);
		console.log("Public key: " + result.publicKey);
		console.log("Keywords: " + result.keywords);
	})
	.catch(error => console.log(error.message));
