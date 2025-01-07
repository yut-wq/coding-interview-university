// ベクトルの実装

#include <stdbool.h>
#include <stdio.h>
#include <strings.h>

// ベクトル型
struct Vector {
  // 配列へのポインタ
  int *pointer_to_array;
  // 配列のアイテムの数
  int size;
  // 配列の長さ(保有上限)
  int capacity;
};

// ベクトルのアイテム数を取得
int size(struct Vector *vector) { return vector->size; }

// ベクトルの保持できるアイテム数を取得
int capacity(struct Vector *vector) { return vector->capacity; }

// ベクトルが空か否かの判定
bool is_empty(struct Vector *vector) {
  if (vector->size == 0) {
    return true;
  } else {
    return false;
  }
}

// サイズを見て、サイズ+1に要素を挿入
// サイズが不足した場合、リサイズ(サイズの拡大)が必要。
// 追加した後に、サイズを加算する
void push(struct Vector *vector, int item) {
  int array_size = vector->size;
  *(vector->pointer_to_array + array_size) = item;
  vector->size = array_size + 1;
}

// 指定したインデックスの値を取得する。
// 範囲外アクセスの場合は爆発する。
int at(struct Vector *vector, int index) {
  // インデックスのチェック
  int vector_size = vector->size;
  if (vector_size <= index) {
    return -999;
  }
  // 要素の取得
  return *(vector->pointer_to_array + index);
}

// 指定したインデックスに値を挿入する
void insert(struct Vector *vector, int index, int item) {
  // インデックスがサイズを超過していないことを確認する
  int vector_size = vector->size;
  if (vector_size <= index) {
    return;
  }

  // 挿入処理
  // 後ろからindexの部分までを後ろにずらす
  for (int vector_index = vector_size - 1; vector_index >= index;
       vector_index--) {
    int old_item = *(vector->pointer_to_array + vector_index);
    *(vector->pointer_to_array + vector_index + 1) = old_item;
  }

  // indexに該当要素を追加する
  *(vector->pointer_to_array + index) = item;
  // サイズの追加
  vector->size = vector_size + 1;
}

// index 0に要素を挿入
void prepend(struct Vector *vector, int item) { insert(vector, 0, item); }

// 最後尾の要素を削除して取得する
int pop(struct Vector *vector) {
  int last_index = vector->size;
  int last_item = at(vector, last_index);

  // 削除した後は初期値である0を入れておく
  insert(vector, last_index, 0);

  // sizeを減らす
  vector->size = vector->size - 1;
  return last_item;
}

// 対象のindexの要素を削除する
// 削除後は全ての要素を左に詰める
void delete(struct Vector *vector, int index) {
  int last_index = vector->size - 1;
  for (int vector_index = index; vector_index < last_index; vector_index++) {
    // 1つ右の要素を代入する
    *(vector->pointer_to_array + vector_index) =
        *(vector->pointer_to_array + vector_index + 1);
  }
  // サイズを減らす
  vector->size -= 1;
}

// itemがvector内にあればindexを返す
// そうでなければ-1を返す
int find(struct Vector *vector, int item) {
  int vector_size = vector->size;
  for (int vector_index = 0; vector_index < vector_size; vector_index++) {
    if (*(vector->pointer_to_array + vector_index) == item) {
      return vector_index;
    }
  }
  return -1;
}

int main() {
  // vectorで参照する先の配列(初期値)
  int actual_array[16];

  // vectorを宣言、初期化
  struct Vector vector = {(int *)&actual_array, 0, 16};
  printf("initialize vector!!\n");

  // vectorのサイズは?
  int vec_size = size(&vector);
  printf("vector size is: %d\n", vec_size);

  // vectorのキャパシティは?
  int vec_cap = capacity(&vector);
  printf("vector capacity is: %d\n", vec_cap);

  // 初期化後は空
  bool is_vec_empty = is_empty(&vector);
  if (!is_vec_empty) {
    printf("error. vector is not empty.");
    return -1;
  }

  // アイテムのプッシュ
  push(&vector, 8);
  // 追加した後はからでない。
  bool is_vec_empty_2 = is_empty(&vector);
  if (is_vec_empty_2) {
    printf("error. vector is empty.");
    return -1;
  }

  // プッシュしたアイテムを取得
  int item_0 = at(&vector, 0);
  printf("item_0: %d\n", item_0);

  // アイテムの挿入
  int new_item = 5;
  insert(&vector, 0, new_item);
  int insert_item = at(&vector, 0);
  printf("insert_item: %d\n", insert_item);

  printf("finish.");

  return 0;
}
