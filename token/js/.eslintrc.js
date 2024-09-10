{
  "root": true,
  "extends": [
      "eslint:recommended",
      "plugin:@typescript-eslint/recommended",
      "plugin:prettier/recommended",
      "plugin:require-extensions/recommended"
  ],
  "parser": "@typescript-eslint/parser",
  "plugins": [
      "@typescript-eslint",
      "prettier",
      "require-extensions"
  ],
  "rules": {
      "@typescript-eslint/ban-ts-comment": "off",
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/no-unused-vars": "off",
      "@typescript-eslint/no-empty-interface": "off",
      "@typescript-eslint/consistent-type-imports": "error"
  },
  "overrides": [
      {
          "files": [
              "examples/**/*",
              "test/**/*"
          ],
          "rules": {
              "require-extensions/require-extensions": "off"
          }
      }
  ]
}
