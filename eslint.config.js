// import standard from 'eslint-config-standard'
import pluginVue from 'eslint-plugin-vue'

export default [
    // standard,
    ...pluginVue.configs['flat/recommended'],
    {

    ignores: [
        "node_modules/**",
        'dist/**',
        'src-tauri/**',
        '.gitignore'
    ],

    rules: {
        "vue/html-self-closing": "off",
        "vue/max-attributes-per-line": "off",
        "comma-dangle": "off",
        "vue/singleline-html-element-content-newline": "off",
        "brace-style": "off",
        "vue/multi-word-component-names": "off",
        "vue/require-default-prop": "off",
        "vue/no-v-html": "off",
    },
}];