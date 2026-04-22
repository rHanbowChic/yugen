
type MusicCategory = "overworld_old" | "overworld_new" | "underwater" | 
"deep_dark" | "creative" | "nether_old" | "nether_new" | "end" | "menu"
import { invoke } from "@tauri-apps/api/core";
import { resolveResource } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";

type MusicJsonSchema = {
    music: Record<MusicCategory, Record<string, { hash: string; size: number }>>;
};

// 从collection中获得分类列表，在music.json搜寻指定分类，把全部这些分类的曲目的路径放在一个array中返回。
export const getMusicFromCollection = async (collection: String) => {
    const collectionCategoryTable: Record<string, MusicCategory[]> = {
        "ultimate": ["overworld_old", "overworld_new", "underwater",
            "deep_dark", "creative", "nether_old", "nether_new", "end", "menu"
        ],
        "overworld": ["overworld_old", "overworld_new"],
        "overworld_classic": ["overworld_old"],
        "overworld_creative": ["overworld_old", "overworld_new", "creative"],
        "creative": ["creative"],
        "deep_dark": ["deep_dark"],
        "underwater": ["underwater"],
        "nether": ["nether_old", "nether_new"],
        "nether_classic": ["nether_old"],
        "end": ["end"],
        "menu": ["menu"],
        "overworld_with_menu": ["overworld_old", "overworld_new", "menu"],
        "overworld_classic_with_menu": ["overworld_old", "menu"],

    }

    const categories = collectionCategoryTable[String(collection)];
    if (!categories) {
        return [] as string[];
    }

    const musicJsonPath = await resolveResource("resources/music.json");
    const contents = await readTextFile(musicJsonPath);
    const typedMusicJson = (JSON.parse(contents)) as MusicJsonSchema;
    const musicPathList: string[] = [];

    for (const category of categories) {
        const categoryMusic = typedMusicJson.music[category];
        if (!categoryMusic) {
            continue;
        }
        musicPathList.push(...Object.keys(categoryMusic));
    }

    return musicPathList;
}

// 使用invoke调用Rust的get_music_bytes指令，获得音乐资源Uint8rray。
/**
 * 注意Rust指令传文件的性能问题。
 */
export const getMusicBytearray = async (musicPath: string): Promise<Uint8Array> => {
    return new Uint8Array(
        await invoke("get_music_bytes", { resourcePath: musicPath })
    )
}

/**
 * 在下载成功后返回。如果文件已经存在，则直接返回。
 */
export const requireMusicDownload = async (musicPath: string): Promise<void> => {
    await invoke("request_music_async", { resourcePath: musicPath })
}

export const getDefaultMinMaxWait = (collection: string): [number, number] => {
    const MIN = 60*1000
    const SEC = 1000
    const collectionWaitTable: Record<string, [number, number]> = {
        "ultimate": [10*MIN, 20*MIN],
        "overworld": [10*MIN, 20*MIN],
        "overworld_classic": [10*MIN, 20*MIN],
        "overworld_creative": [1*MIN, 3*MIN],
        "creative": [1*MIN, 3*MIN],
        "deep_dark": [10*MIN, 20*MIN],
        "underwater": [10*MIN, 20*MIN],
        "nether": [1*MIN, 2*MIN],
        "nether_classic": [1*MIN, 2*MIN],
        "end": [5*MIN, 20*MIN],
        "menu": [1*SEC, 30*SEC],
        "overworld_with_menu": [1*SEC, 30*SEC],
        "overworld_classic_with_menu": [1*SEC, 30*SEC],
    }
    const result = collectionWaitTable[collection]
    if (!result) {
        return [10*MIN, 20*MIN]
    }
    return result
}