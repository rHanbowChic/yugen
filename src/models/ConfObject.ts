
// collection: 很多music.json中分类(category)的集合，也对应了各自的默认随机等待时长
// minWait与maxWait: 毫秒
export class ConfObject {
    public collection: string;
    public minWait: number;
    public maxWait: number;

    constructor (collection: string = "overworld", minWait: number = 10*60*1000, maxWait: number = 20*60*1000){
        this.collection = collection;
        this.minWait = minWait;
        this.maxWait = maxWait;
    }
}