// подключение фреймворка
const Excalibur = require("../JavaScript/Excalibur.js");
// создание экземпляра класса
let hashgraph = new Excalibur("_nodeAddress_", "_nodeAccount_", true);
// Вывод переменных внутри класса
console.log(hashgraph);
// Вывод методов доступных для работы
console.log(hashgraph.__proto__);
// Вывод информации о версиях SDK и фреймворка
console.log(hashgraph.getVersions().hederaSDK);
console.log(hashgraph.getVersions().xclbrFramework); 