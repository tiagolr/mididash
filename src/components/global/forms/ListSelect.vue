<!--
  List multiselect with checkboxes per option
  Use :placeholder prop to set the input text

  [input search]
  [checkbox option1.label]
  [checkbox option2.label]
  [checkbox option3.label]
  [checkbox option4.label]
-->
<script>
import Checkbox from './Checkbox.vue'
export default {

  components: {
    Checkbox,
  },

  props: {
    selected: {
      type: Array,
      required: true,
    },
    options: {
      type: Array,
      required: true,
      default: () => [], // [{ id: string, label: string }]
    },
    placeholder: {
      type: String,
      required: false,
      default: 'Select an option',
    },
    disabled: {
      type: Boolean,
      required: false,
      default: false,
    },
    showDropdownIcon: {
      type: Boolean,
      default: true
    },
    closeOnSelect: Boolean,
    hideCheckboxes: Boolean,
    isLoading: Boolean,
    allButton: Boolean,
    alignTop: Boolean,
    noOptionsText: String,
  },

  emits: [
    'selected',
    'search',
    'open',
    'clear'
  ],

  data () {
    return {
      optionsVisible: false,
      searchFilter: '',
      selectIndex: -1
    }
  },

  computed: {
    filteredOptions: vm => !vm.searchFilter.length
      ? vm.options
      : vm.options
        .filter(o => o.label.toLowerCase().includes(vm.searchFilter.toLowerCase())),
    allSelected: vm => vm.selected.length === vm.options.length
  },

  watch: {
    searchFilter (s) {
      this.$emit('search', s)
    }
  },

  mounted () {
    document.addEventListener('click', this.onClickOutside, { capture: true })
    document.addEventListener('keydown', this.onKeydown, { capture: true })
    document.addEventListener('focus', this.onClickOutside, { capture: true })
  },

  beforeUnmount () {
    document.removeEventListener('click', this.onClickOutside, { capture: true })
    document.removeEventListener('keydown', this.onKeydown, { capture: true })
    document.removeEventListener('focus', this.onClickOutside, { capture: true })
  },

  methods: {
    clear () {
      this.exit()
      this.selected.forEach(s => {
        this.$emit('selected', s)
      })
      this.$emit('clear')
    },
    selectOption (option) {
      this.$emit('selected', option.id)
      this.showOptions()
      this.$refs.input.focus()
      if (this.closeOnSelect) {
        this.exit()
      }
    },
    showOptions () {
      if (!this.disabled) {
        this.optionsVisible = true
        this.$emit('open')
      }
    },
    toggleAll () {
      if (this.allSelected) {
        this.options.forEach(o => this.selectOption(o))
      } else {
        this.options
          .filter(o => !this.selected.includes(o.id))
          .forEach(o => this.selectOption(o))
      }
    },
    onClickOutside (e) {
      if (this.optionsVisible && !this.$refs.select.contains(e.target)) {
        this.exit()
      }
    },
    exit () {
      this.selectIndex = 0
      this.searchFilter = ''
      this.optionsVisible = false
      this.$refs.input.blur()
    },
    onKeydown (event) {
      if (!this.optionsVisible) return
      if (event.key === 'Enter' && this.filteredOptions[this.selectIndex]) {
        this.selectOption(this.filteredOptions[this.selectIndex])
      }
      else if (event.key === 'ArrowUp') {
        this.selectIndex -= 1
        if (this.selectIndex < 0) this.selectIndex = this.filteredOptions.length - 1
      }
      else if (event.key === 'ArrowDown') {
        this.selectIndex += 1
        if (this.selectIndex >= this.filteredOptions.length) this.selectIndex = 0
      }
      else if (event.key === 'Escape') {
        this.exit()
        this.$refs.input.blur()
        event.stopPropagation()
        event.preventDefault()
      }
      if (this.$refs.option?.[this.selectIndex]) {
        this.$refs.option[this.selectIndex].scrollIntoViewIfNeeded()
      }
    },
  },
}
</script>
<template>
  <div ref="select" class="list-select" tabindex="0">
    <div class="input-wrapper">
      <input
        ref="input"
        v-model="searchFilter"
        :disabled="disabled"
        :placeholder="placeholder"
        autocomplete="off"
        :class="selected.length && 'selected'"
        @focus="showOptions"
      />
      <svg
        v-show="showDropdownIcon"
        class="dropdown-icon"
        width="20"
        height="20" viewBox="0 0 5.292 5.292" xmlns="http://www.w3.org/2000/svg" @click="showOptions()"
      >
        <path style="fill:var(--copy-light);stroke:none;" d="m1.302 3.046-.525-.91-.526-.911H2.354l-.526.91z" transform="translate(.799 -.383) scale(1.41818)" />
      </svg>
    </div>
    <div v-if="optionsVisible" ref="list" class="list" :class="alignTop && 'align-top'">
      <div v-if="allButton && filteredOptions.length === options.length && options.length" class="item" :checked="allSelected" @click="toggleAll">
        <checkbox :checked="allSelected">
        </checkbox>
        <div>{{ $t('other.all') }}</div>
      </div>
      <div
        v-for="(option, index) in filteredOptions"
        :key="index"
        ref="item"
        class="item"
        :class="selectIndex === index && 'selected'"
        @mouseenter="selectIndex = index"
        @mouseleave="() => {if (selectIndex === index) selectIndex = -1;}"
        @click="selectOption(option)"
      >
        <checkbox v-if="!hideCheckboxes" :checked="selected.includes(option.id)" @focus="selectIndex = index">
        </checkbox>
        <div :title="option.label || option.id || '-'">{{ option.label || option.id || '-' }}</div>
      </div>
      <div v-if="filteredOptions.length === 0" class="item">
        {{ noOptionsText || 'No options' }}
      </div>
    </div>
  </div>
</template>

<style scoped>

.list-select {
  width: 100%;
  height: calc(2rem);
  position: relative;
  margin: 0;
}

.dropdown-icon {
  position: absolute;
  bottom: 5px;
  right: 7px;
  cursor: pointer;
}
.input-wrapper {
  width: 100%;
  position: relative;
  display: inline-block;
  height: 100%;
  border: 1px solid var(--copy-light);
  border-radius: 4px;
}
input {
  all: unset;
  padding: 0 10px;
  background: transparent;
  outline: none;
  width: 100%;
  height: 100%;
  border: none;
  color: var(--copy)
}
input:disabled {
  opacity: 0.5;
}
input::placeholder {
  color: var(--copy-lighter)
}
input.selected::placeholder {
  color: var(--copy)
}

.list {
  position: absolute;
  left: 0;
  z-index: 1000;
  width: 100%;
  margin: 0;
  list-style: none;
  text-align: left;
  max-height: 250px;
  overflow: auto;
  border-radius: 4px;
  background: var(--foreground-lighter);

}

.list.align-top {
  transform: translateY(calc(-100% - 40px));
}

.item {
  cursor: pointer;
  overflow: hidden;
  position: relative;
  margin: 0;
  display: flex;
  padding: var(--panel-padding);
  align-items: center;
  gap: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item.selected {
  background: var(--primary);
}
</style>
