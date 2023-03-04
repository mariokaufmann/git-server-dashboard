const fs = require('fs');
const path = require('path');

const args = process.argv;
if (args.length !== 4) {
  throw new Error(
    'Pass the folder and component name as arguments (in kebab-case)'
  );
}

const folder = args[2];
const componentName = args[3];

const componentPath = path.join(folder, componentName);
fs.mkdirSync(componentPath);

const camelCaseFileName = convertKebabCaseToCamelCase(componentName);

const componentFile = path.join(componentPath, `${camelCaseFileName}.tsx`);
fs.writeFileSync(componentFile, getComponentFileContents(camelCaseFileName), {
  flag: 'wx',
});

const styleFile = path.join(componentPath, `${camelCaseFileName}.module.css`);
fs.writeFileSync(styleFile, getStyleFileContents(camelCaseFileName), {
  flag: 'wx',
});

function convertKebabCaseToCamelCase(componentName) {
  const parts = componentName.split('-');
  const camelCaseParts = parts.map((part) => capitalizeFirstLetter(part));
  return camelCaseParts.join('');
}

function capitalizeFirstLetter(string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

function getComponentFileContents(componentName) {
  return `import { Component } from "solid-js";
import styles from "./${componentName}.module.css";
const ${componentName}: Component = () => {
  return (
    <div class=\{styles.${componentName}\}></div>
  );
};
export default ${componentName};
  `;
}

function getStyleFileContents(componentName) {
  return `.${componentName} \{\}`;
}
