import { ref, computed } from "vue";
import { messages, type Locale } from "../locales";

const STORAGE_KEY = "locale";

const locale = ref<Locale>((localStorage.getItem(STORAGE_KEY) as Locale) || "zh");

const fallbackLocale: Locale = "en";

function setLocale(l: Locale) {
  locale.value = l;
  localStorage.setItem(STORAGE_KEY, l);
}

function get(obj: any, path: string) {
  return path.split(".").reduce((o, k) => o?.[k], obj);
}

function format(str: string, params?: Record<string, any>) {
  if (!params) return str;
  return str.replace(/\{(\w+)\}/g, (_, k) => params[k] ?? "");
}

function t(key: string, params?: Record<string, any>) {
  let msg = get(messages[locale.value], key);
  if (!msg) {
    msg = get(messages[fallbackLocale], key);
  }
  return msg ? format(msg, params) : key;
}

function getLocales() {
  return Object.keys(messages) as Locale[];
}

export function useI18n() {
  return {
    locale: computed(() => locale.value),
    setLocale,
    getLocales,
    t,
  };
}
