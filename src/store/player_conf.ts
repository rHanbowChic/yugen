import { reactive, watch } from "vue";

let STORAGE_KEY = "player_conf"

const data = reactive({
    collection: "overworld",
    minWait: 10*60*1000,
    maxWait: 20*60*1000,
})


if (localStorage.getItem(STORAGE_KEY) !== null) {
    try {
        const raw = JSON.parse(localStorage.getItem(STORAGE_KEY) as string)
        Object.assign(data, raw)
    }
    catch (e) {
        console.log(`${STORAGE_KEY} 解析失败，使用默认值`)
    }
}

watch(data, (_newValue: any, _oldValue: any) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data))
})

export default data;