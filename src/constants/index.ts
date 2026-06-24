const MIN = 60*1000
const SEC = 1000

type MusicCategory = "overworld_old" | "overworld_new" | "underwater" | 
"deep_dark" | "creative" | "nether_old" | "nether_new" | "end" | "menu"

type HexColor = `#${string}`

type CollectionConfigItem = {
    categories: MusicCategory[],
    minWait: number,
    maxWait: number,
    bgColor: HexColor
}

export const COLLECTIONS: Record<string, CollectionConfigItem> = {
    "ultimate": {
        categories:["overworld_old", "overworld_new", "underwater",
        "deep_dark", "creative", "nether_old", "nether_new", "end", "menu"
        ],
        minWait: 10*MIN,
        maxWait: 20*MIN,
        bgColor: "#A8C7B8", // 柔和草绿色
    },
    "overworld": {
        categories:["overworld_old", "overworld_new"],
        minWait: 10*MIN,
        maxWait: 20*MIN,
        bgColor: "#A9C8A2", // 草地绿色
    },
    "overworld_classic": {
        categories:["overworld_old"],
        minWait: 10*MIN,
        maxWait: 20*MIN,
        bgColor: "#B8C99A", // 怀旧草原色
    },
    "overworld_creative": {
        categories:["overworld_old", "overworld_new", "creative"],
        minWait: 1*MIN,
        maxWait: 3*MIN,
        bgColor: "#B8D8F0", // 天空蓝
    },
    "creative": {
        categories:["creative"],
        minWait: 1*MIN,
        maxWait: 3*MIN,
        bgColor: "#D6C4F2", // 梦幻淡紫
    },
    "deep_dark": {
        categories:["deep_dark"],
        minWait: 10*MIN,
        maxWait: 20*MIN,
        bgColor: "#6B8A92", // 幽暗青蓝
    },
    "underwater": {
        categories:["underwater"],
        minWait: 10*MIN,
        maxWait: 20*MIN,
        bgColor: "#8CC7D8", // 海洋蓝
    },
    "nether": {
        categories:["nether_old", "nether_new"],
        minWait: 1*MIN,
        maxWait: 2*MIN,
        bgColor: "#D78A7A", // 柔和熔岩红
    },
    "nether_classic": {
        categories:["nether_old"],
        minWait: 1*MIN,
        maxWait: 2*MIN,
        bgColor: "#C97A6B", // 经典地狱砖红
    },
    "end": {
        categories:["end"],
        minWait: 5*MIN,
        maxWait: 20*MIN,
        bgColor: "#C8B8E8", // 末地淡紫
    },
    "menu": {
        categories:["menu"],
        minWait: 1*SEC,
        maxWait: 30*SEC,
        bgColor: "#E3DCCF", // 温暖米色
    },
    "overworld_with_menu": {
        categories:["overworld_old", "overworld_new", "menu"],
        minWait: 1*SEC,
        maxWait: 30*SEC,
        bgColor: "#B7D0AA", // 草地 + UI
    },
    "overworld_classic_with_menu": {
        categories:["overworld_old", "menu"],
        minWait: 1*SEC,
        maxWait: 30*SEC,
        bgColor: "#C5D2A6", // 经典草地色
    },
}