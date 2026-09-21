# `OuDia.1.02` File Format

## Outline

- `RosenFileData`
	- `Rosen`
		- `Eki[]`
		- `Ressyasyubetsu[]`
		- `Dia[]`
			- `Kudari`
				- `Ressya`
			- `Nobori`
				- `Ressya`
	- `DispProp`

`OuDia.1.02` 形式は、 `OuPropertiesText`記法を用いてダイヤグラムの各要素をシリアル化します。

## OuPropertiesText

`OuPropertiesText`は、`.ini`形式を階層構造に対応できるように拡張した格好のフォーマットです。
以下の、2つの要素から構成されます。

- Directory
- Property

Directoryは、階層構造を表す要素です。`{DirectoryKey}.`で開始し、複数の要素を記述し、`.`で終了します。DirectoryKeyに`=`を含むことはできません。

Propertyは、1つのKey-Value構造を表す要素であり、`Key`文字列と、`Value`文字列から構成されます。Keyに`=`を含むことはできませんが、Valueに含むことはできます。また、Valueに改行が含まれる場合には、改行文字をエスケープし`\n`に置換します。Valueにバックスラッシュ`\`が含まれる場合には、エスケープとして`\\`に置換します。

```ebnf
(* OuPropertiesTextのファイルは、要素の配列によって構成される *)
File          ::= ElementList ;

(* 要素の配列は、1つ以上の要素で構成される *)
ElementList   ::= Element { Element } ;
(* 要素は PropertyとDirectoryによって構成される *)
Element       ::= Property | Directory ;

(* Propertyは `{Key}={Value}` の並びで構成される *)
Property      ::= Key Eq Value Newline ;
Eq            ::= "=" ;

(* Directoryは、`{DirectoryKey}.` によって開始し、要素の配列が続き、 `.` によって終了する *)
Directory     ::= DirectoryKey Dot Newline ElementList Dot Newline ;
Dot           ::= "." ;

(* PropertyのKeyには、Equalを除くAlphabetが使用できる *)
Key           ::= { Alphanumeric - "=" } ;
(* Valueには、改行を除く全ての文字が使用できる *)
Value         ::= { Character - "\r\n" } ;
(* DirectoryNameには、Equalを除くAlphabetが使用できる *)
DirectoryKey  ::= { Alphanumeric - "=" } ;
(* 改行コードはCRLFである *)
Newline       ::= "\r\n" ;
```

## RosenFileData Directory

1つの`.oud`ファイルあるいは`.oud2`ファイルを表します。

`RosenFileData` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|FileType|String (Property)|Required|ファイル形式を表します。|
|Rosen|[Rosen](#rosen-directory) (Directory)|Required|1つの路線情報を表します。|
|DispProp|[DispProp](#dispprop-directory) (Directory)|Required|表示設定情報を表します。|
|FileTypeAppComment|String (Property)|Optional|ファイルがどのアプリケーションで作成されたかを表します。|

## Rosen Directory

`.oud`ファイルに保存されている路線情報を表します。

`Rosen` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|Rosenmei|String (Property)|Optional|路線名を表します。|
|Eki|[Eki](#eki-directory)\[\] (Directory)|Optional|駅を表します。この要素は複数含むことができます。|
|Ressyasyubetsu|[Ressyasyubetsu](#ressyasyubetsu-directory)\[\] (Directory)|Optional|列車種別を表します。この要素は、複数含むことができます。|
|Dia|[Dia](#dia-directory)\[\] (Directory)|Optional|時刻表を表します。この要素は、複数含むことができます。|
|KitenJikoku|[Jikoku](#jikoku-property) (Property)|Optional|ダイヤグラム描画の起点時刻を表します。|
|DiagramDgrYZahyouKyoriDefault|Unsigned Int (Property)|Optional|列車が設定されていない時の、次駅までの秒相当の駅間距離|
|Comment|String (Property)|Optional|自由に記述できるコメントを表します。|

## Eki Directory

1つの駅(停車場、停留場、信号場等)を表します。

`Eki` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|Ekimei|String (Property)|Optional|駅名を表します。また、空文字を認めます。|
|Ekijikokukeisiki|[Ekijikokukeisiki](#ekijikokukeisiki-property) (Property)|Optional|時刻表での駅時刻の表示形式を表します。|
|Ekikibo|[Ekikibo](#ekikibo-property) (Property)|Optional|時刻表やダイヤグラムでの規模による駅表示形式を表します。|
|Kyoukaisen|Boolean (Property)|Optional|時刻表上で、この駅の下り側に境界線を描画するかどうかを表します。|
|DiagramRessyajouhouHyoujiKudari|[DiagramRessyajouhouHyouji](#diagramressyajouhouhyouji-property) (Property)|Optional|(不明)|
|DiagramRessyajouhouHyoujiNobori|[DiagramRessyajouhouHyouji](#diagramressyajouhouhyouji-property) (Property)|Optional|(不明)|

### Ekijikokukeisiki Property

時刻表での駅時刻の表示形式を表します。

`Ekijikokukeisiki` Propertyは、以下のいずれかの値をとります。

- `Jikokukeisiki_Hatsu`
- `Jikokukeisiki_Hatsuchaku`
- `Jikokukeisiki_KudariChaku`
- `Jikokukeisiki_NoboriChaku`

### Ekikibo Property

時刻表やダイヤグラムでの規模による駅表示形式を表します。

`Ekikibo` Propertyは、以下のいずれかの値をとります。

- `Ekikibo_Ippan`
- `Ekikibo_Syuyou`

### DiagramRessyajouhouHyouji Property

`DiagramRessyajouhouHyouji` Propertyは、以下のいずれかの値をとります。

- `DiagramRessyajouhouHyouji_Anytime`
- `DiagramRessyajouhouHyouji_Not`

## Ressyasyubetsu Directory

1つの列車種別を表します。

`Ressyasyubetsu` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|Syubetsumei|String (Property)|Required|列車種別名を表します。この項目は必須です。|
|Ryakusyou|String (Property)|Optional|列車種別の略称名を表します。|
|JikokuhyouMojiColor|[Color](#color-property) (Property)|Optional|時刻表における列車時刻の描画文字色を表します。|
|JikokuhyouFontIndex|Unsigned Int (Property)|Optional|時刻表における描画フォントインデックスを表します。|
|DiagramSenColor|[Color](#color-property) (Property)|Optional|ダイヤグラムにおける列車線の描画色を表します。|
|DiagramSenStyle|[SenStyle](#senstyle-property) (Property)|Optional|ダイヤグラムにおける列車線の描画線種を表します。|
|DiagramSenIsBold|Boolean (Property)|Optional|ダイヤグラムにおける列車線を太線で描画するかどうかを表します。|
|StopMarkDrawType|[StopMarkDrawType](#stopmarkdrawtype-property) (Property)|Optional|(不明)|

### SenStyle Property

ダイヤグラムにおける列車線の描画線種を表します。

`SenStyle` Propertyは、以下のいずれかの値をとります。

- `SenStyle_Jissen`
- `SenStyle_Hasen`
- `SenStyle_Tensen`
- `SenStyle_Ittensasen`

### StopMarkDrawType Property

`StopMarkDrawType` Propertyは、以下のいずれかの値をとります。

- `EStopMarkDrawType_DrawOnStop`
- `EStopMarkDrawType_Nothing`
- `EStopMarkDrawType_DrawOnPass`

## Dia Directory

1つの時刻表を表します。

`Dia` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|DiaName|String (Property)|Required|時刻表名を表します。この項目は必須です。|
|Kudari|[Ressya](#ressya-directory)\[\]|Required|時刻表に含まれる下り列車の一覧を表します。`Kudari Directory`の子要素として`Ressya Directory`を複数含むことができます。|
|Nobori|[Ressya](#ressya-directory)\[\]|Required|時刻表に含まれる下り列車の一覧を表します。`Nobori Directory`の子要素として`Ressya Directory`を複数含むことができます。|

## Ressya Directory

1つの列車を表します。

`Ressya` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|Houkou|[Houkou](#houkou-property) (Property)|Required|列車の運転方向を表します。この項目は必須です。|
|Syubetsu|Unsigned Int (Property)|Optional|列車の列車種別インデックスを表します。|
|Ressyabangou|String (Property)|Optional|列車番号を表します。この項目は一意である必要はありません。|
|Ressyamei|String (Property)|Optional|列車名を表します。|
|Gosuu|String (Property)|Optional|列車の号数を表します。|
|EkiJikoku|[EkiJikoku](#ekijikoku-property) (Property)|Optional|列車の駅時刻一覧を表します。|
|Bikou|String (Property)|Optional|列車の備考を表します。|

### Houkou Property

列車の運転方向を表します。

`Houkou` Propertyは、以下のいずれかの値をとる。

- `Kudari`
- `Nobori`

### EkiJikoku Property

列車に含まれる駅時刻のリストを表します。

`EkiJikoku` Propertyはカンマ区切りの1次元配列の値として表されます。

以下のいずれかの形式を用いて駅時刻を指定します。

- ` ` (空文字) の場合 運行なし
- `{unsigned integer}` の場合 駅扱いのみ指定
- `{unsigned integer};{Jikoku}` の場合 駅扱い、発車時刻のみ指定
- `{unsigned integer};{Jikoku}/` の場合 駅扱い、到着時刻のみ指定
- `{unsigned integer};{Jikoku}/{Jikoku}` の場合 駅扱い、到着時刻、発車時刻を指定

## DispProp Directory

表示設定情報を表します。

`DispProp` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|JikokuhyouFont|[FontProp](#fontprop-property)\[8\] (Property)|Required|時刻表の列車時刻に使用されるフォント設定を表します。8要素に満たない場合はデフォルト値を代入します。|
|JikokuhyouVFont|[FontProp](#fontprop-property) (Property)|Optional|時刻表上の縦書き部分に使用されるフォント設定を表します。|
|DiaEkimeiFont|[FontProp](#fontprop-property)(Property)|Required|ダイヤグラム上の駅名表示に使用されるフォント設定を表します。|
|DiaJikokuFont|[FontProp](#fontprop-property) (Property)|Required|ダイヤグラム上の時刻表示に使用されるフォント設定を表します。|
|DiaRessyaFont|[FontProp](#fontprop-property) (Property)|Optional|ダイヤグラム上の列車表示に使用されるフォント設定を表します。|
|CommentFont|[FontProp](#fontprop-property) (Property)|Required|コメントに使用されるフォント設定を表します。|
|DiaMojiColor|[Color](#color-property) (Property)|Optional|ダイヤグラム上の文字表示に使用される文字色を表します。|
|DiaHaikeiColor|[Color](#color-property) (Property)|Optional|ダイヤグラムの背景表示に使用される色を表します。|
|DiaRessyaColor|[Color](#color-property) (Property)|Optional|ダイヤグラムの列車表示に使用される既定色を表します。|
|DiaJikuColor|[Color](#color-property)\ (Property)|Optional|ダイヤグラムの軸表示に表示される色を表します。|
|EkimeiLength|Unsigned Int (Property)|Optional|ダイヤグラム上の駅名表示欄の桁数を表します。|
|JikokuhyouRessyaWidth|Unsigned Int (Property)|Optional|時刻表上での列車表示欄の桁数を表します。|
|DiaRessyajouhouHyoujiEkiOrderKudari|String (Property)|Optional|未使用|
|DiaRessyajouhouHyoujiEkiOrderNobori|String (Property)|Optional|未使用|


## Color Property

ARGBからなる色情報を表します。

16進数で`AARRGGBB`形式でシリアル化されます。

## FontProp Property

1つの書体にまつわる情報を表します。

`FontProp`では、1行でディレクトリを表現する形式が用いられる。具体的には、`{KEY}={VALUE}`方式のPropertyを複数表すため、`;`(セミコロン)を用いて連結している。KEYまたはVALUEに`;`が含まれていた場合の挙動は不明です。

`FontProp` Directoryには、以下の要素が含まれます。

|Key|Type|Required|概要|
|---|---|---|---|
|PointTextHeight|Unsigned Int (Property)|Optional||
|LogicalunitTextHeight|Unsigned Int (Property)|Optional||
|LogicalunitCellHeight|Unsigned Int (Property)|Optional||
|Facename|String (Property)|Required|書体名を表す。空文字の場合エラーとなる。|
|Bold|Boolean (Property)|Optional||
|Itaric|Boolean (Property)|Optional||
|Underline|Boolean (Property)|Optional||
|StrikeOut|Boolean (Property)|Optional||
|Escapement|Unsigned Int (Property)|Optional||

## Jikoku Property

`0:00:00`からの経過時間をもって時刻を表すプロパティです。時、分、秒の3要素から構成されます。

`Jikoku` Propertyは、以下の文法によって時刻を表します。

```ebnf

Jikoku	::= TimeWithColon | TimeWithoutColon;

JikokuWithColon		::= Hour Colon Minute [Colon Second];
JikokuWithoutColon	::= Hour Minute [Second];

(* 「時」を表す部分 *)
Hour	::= Digit | Digit Digit | " " Digit;

(* 「分」を表す部分 *)
Minute	::= Digit Digit;

(* 「秒」を表す部分 *)
Second	::= Digit Digit;

(* 時刻表示に使用できる数字 *)
Digit	::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

Colon	::= ":";
```

## Boolean Property

真偽値を表します。

`Boolean` Propertyは以下のいずれかの値をとります。

- `1` true, 真
- (`1`以外の文字列) false, 偽
