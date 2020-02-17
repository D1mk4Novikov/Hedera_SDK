'use strict';

// Import a configuration file and it's binding to a constant for working with it
const Settings = require("./configuration/settings.json");

// Import a "package.json" file and it's binding to a constant for reading library version
const Package = require("./package.json");

// Import Foreign Functions Interface for working with methods from `library.os_ext`
const NodeFFI = require("ffi");



// The `Excalibur_` class which includes constructor and methods for working with `library.os_ext`
function Excalibur(nodeAddress, nodeAccount, isCustomBuildType = false, selectedOS = "macOS") {
	
	// Creating a variable and storing the address to the hedera hashgraph node in it
	this.nodeAddress = nodeAddress;
	// Creating a variable and storing the port to the hedera hashgraph node in it
	this.nodeAccount = nodeAccount;
	// Getting the key to fetch the value in JSON depending on the type of assembly Hedera SDK
	this.buildType = (isCustomBuildType === true) ? ("CustomBuild" + selectedOS) : selectedOS;
	// Getting path to the `library.os_ext`
	this.targetOS = (Settings[this.buildType]) ? Settings[this.buildType] : console.log(`Invalid key for importing library [${selectedOS}...]`);
	
	// Importing methods from `library.os_ext`
	this.library = NodeFFI.Library(this.targetOS, {
		get_account_info: ["void", ["string", "string", "string", "string"]],
		//create_file_from_file: ["string", ["string", "string", "string", "string", "string"]],
		//create_contract: ["string", ["string", "string", "string", "string", "string", "string"]],
		//call_contract: ["string", ["string", "string", "string", "string", "string", "string", "string", "string"]],
		generate_keys: ["string", []],
		get_sdk_version: ["string", []]
		// сюда дописать 6-7 методов для выгрузки
	});
	
}



// This method allows you to get information about your current account
Excalibur.prototype.getAccountInfo = function (userAccount, userPrivateKey) {
	this.library.get_account_info(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey);
	/*
	return new Promise((resolve, reject) => {
		let AccountInfo = JSON.parse(this.library.get_account_info(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey));
		if (Object.keys(AccountInfo).length !== 0) {
			resolve(AccountInfo);
		} else {
			reject(new Error("Failed to get information about current account"));
		}
	});
	*/
	// да блять хуле не работает достало почему так нахуй
}


// This method allows you to create a file from another file by specifying the path to it
/*
Excalibur.prototype.createFileFromFile = function (userAccount, userPrivateKey, pathToFile) {
	return this.library.create_file_from_file(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, pathToFile);
	return new Promise((resolve, reject) => {
		let FileInfo = JSON.parse(this.library.create_file_from_file(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, pathToFile));
		if (Object.keys(FileInfo).length !== 0) {
			resolve(FileInfo);
		} else {
			reject(new Error("Failed to create a new file from another file"));
		}
	});
}


/*
// This method allows you to create a contract
Excalibur.prototype.createContract = function (userAccount, userPrivateKey, fileID, gasValue) {
	return new Promise((resolve, reject) => {
		let ContractInfo = JSON.parse(this.library.create_contract(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, fileID, gasValue));
		if (Object.keys(ContractInfo).length !== 0) {
			resolve(ContractInfo);
		} else {
			reject(new Error("Failed to create a new contract"));
		}
	});
}


// This method allows you to call a smart contract using a method from it
Excalibur.prototype.callContract = function (userAccount, userPrivateKey, contractID, gasValue, pathToABI, methodName) {
	return new Promise((resolve, reject) => {
		let CallInfo = JSON.parse(this.library.call_contract(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, contractID, gasValue, pathToABI, methodName));
		if (Object.keys(CallInfo).length !== 0) {
			resolve(CallInfo);
		} else {
			reject(new Error("Failed to call a smart contract"));
		}
	});
}
*/


// This method allows you to generate private/public key
Excalibur.prototype.generateKeys = function () {
	return new Promise((resolve, reject) => {
		let Keys = JSON.parse(this.library.generate_keys());
		if (Object.keys(Keys).length !== 0) {
			resolve(Keys);
		} else {
			reject(new Error("Failed to generate private/public key"));
		}
	});
}


// тут дописать еще 6-7 методов
// этот фреймворк на данный момент еще не для выгрузки


// This method allows you to get information about this framework and Hedera SDK based on Rust
Excalibur.prototype.getVersions = function () {
	let Versions = new Object;
	Versions.xclbrFramework = "Excalibur_ frmwk ver. " + Package.version;
	Versions.hederaSDK = "Rust Hedera SDK ver. " + this.library.get_sdk_version();
	return Versions;
}



// Export framework for using in other projects
module.exports = Excalibur;