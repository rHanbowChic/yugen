
type MusicCategory = "overworld_old" | "overworld_new" | "underwater" | 
"deep_dark" | "creative" | "nether_old" | "nether_new" | "end" | "menu"
import { COLLECTIONS } from "@/constants";
import { invoke } from "@tauri-apps/api/core";
import { resolveResource } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";

type MusicJsonSchema = {
    music: Record<MusicCategory, Record<string, { hash: string; size: number }>>;
};

// 从collection中获得分类列表，在music.json搜寻指定分类，把全部这些分类的曲目的路径放在一个array中返回。
export const getMusicFromCollection = async (collection: string) => {
    

    const categories = COLLECTIONS[collection]?.categories;
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
    const minWait = COLLECTIONS[collection]?.minWait
    const maxWait = COLLECTIONS[collection]?.maxWait
    if (!minWait || !maxWait) {
        return [600, 1200]
    }
    return [minWait, maxWait]
}