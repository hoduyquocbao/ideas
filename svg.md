thiết kế một cấu trúc thư mục và các khái niệm cốt lõi cho dự án editor SVG sử dụng TypeScript và WGPU, tập trung vào hiệu suất cao, zero-copy, và an toàn luồng với Web Workers, đồng thời tuân thủ các quy tắc đặt tên nghiêm ngặt mà bạn đã đề ra, được điều chỉnh theo triết lý BEARD.

**Tổng quan Triết lý BEARD Áp dụng:**
Triết lý BEARD nhấn mạnh kiến trúc tiến hóa và khả năng bảo trì. Trong dự án này:

  * **Cấu trúc Module:** Lấy cảm hứng từ sự phân tách rõ ràng của Atomic Design, chúng ta sẽ tổ chức mã nguồn thành các module với trách nhiệm cụ thể, từ tương tác cấp thấp với WGPU đến logic nghiệp vụ của editor.
  * **Tiến hóa Backend (Workers):** Logic chạy trong Web Workers sẽ được thiết kế để có thể thay đổi và mở rộng độc lập, tương tự như các microservices hoặc module tự trị trong kiến trúc backend tiến hóa.
  * **Kết nối FE/BE (Main Thread/Workers):** Sự tương tác giữa luồng chính (quản lý UI và WGPU context) và các Web Workers (xử lý tác vụ nặng) sẽ được thiết kế cẩn thận, đặc biệt là về giao tiếp dữ liệu (zero-copy).

**1. Cấu trúc Thư mục Đề xuất:**

Dưới đây là cấu trúc thư mục gốc `src` được đề xuất, với các module chính:

```
src/
├── core/                    # Tương tác WGPU, pipeline render, tiện ích đồ họa cấp thấp
│   ├── engine/              # Abstraction chính của WGPU (device, queue, swapchain)
│   ├── buffer/              # Quản lý GPU buffer, tiện ích zero-copy
│   ├── shader/              # Tải và quản lý shader (WGSL)
│   ├── pipeline/            # Thiết lập render pipeline
│   ├── texture/             # Xử lý texture
│   └── resource/            # Các tài nguyên GPU khác (sampler, bind group layout)
│
├── svg/                     # Phân tích, biểu diễn và xử lý SVG
│   ├── dom/                 # Biểu diễn cấu trúc SVG (tương tự DOM)
│   ├── parser/              # Phân tích chuỗi SVG thành cấu trúc DOM nội bộ
│   ├── path/                # Phân tích và xử lý dữ liệu path SVG
│   ├── style/               # Phân tích và áp dụng style SVG
│   └── geometry/            # Tính toán hình học, tessellation cho rendering
│
├── editor/                  # Logic đặc thù của editor, trạng thái UI, công cụ
│   ├── state/               # Quản lý trạng thái ứng dụng, lịch sử, vùng chọn
│   ├── tool/                # Các công cụ chỉnh sửa (chọn, vẽ đường, hình khối)
│   ├── command/             # Command pattern cho undo/redo
│   └── view/                # Quản lý viewport (zoom, pan)
│
├── worker/                  # Mã nguồn liên quan đến Web Worker
│   ├── manager/             # Quản lý worker pool, phân phối tác vụ
│   ├── task/                # Định nghĩa các tác vụ offload cho worker (tessellation, tính toán phức tạp)
│   └── serialize/           # (De)serialization cho message worker (tập trung zero-copy)
│
├── platform/                # Tích hợp đặc thù của trình duyệt
│   ├── input/               # Xử lý sự kiện chuột, bàn phím
│   └── file/                # Truy cập file hệ thống (nếu có)
│
├── ui/                      # Các thành phần giao diện người dùng (nếu không dùng framework ngoài)
│   ├── atom/                # Các phần tử UI cơ bản (button, input) - tuân thủ BEARD naming
│   ├── molecule/            # (Tương tự Atomic Design)
│   └── organism/            # (Tương tự Atomic Design)
│
└── common/                  # Tiện ích, kiểu dữ liệu, hằng số dùng chung
    ├── type/                # Các TypeScript type, interface (chỉ định nghĩa cấu trúc dữ liệu)
    ├── constant/            # Hằng số toàn cục
    ├── math/                # Thư viện toán học (vector, matrix - nếu tự viết)
    └── error/               # Các kiểu lỗi tùy chỉnh

```

**2. Quy ước Đặt tên (Theo BEARD và Yêu cầu):**

Tất cả các định danh do chúng ta tạo ra sẽ tuân thủ nghiêm ngặt quy tắc "một từ đơn, đầy đủ ý nghĩa" và các quy tắc BEARD khác.

  * **`fields` (Thuộc tính của class/object):**

      * Mô tả: Tên thuộc tính trong class hoặc object.
      * Quy tắc: Một từ đơn, có nghĩa, thường là danh từ. Viết thường (lowercase) hoặc theo quy ước của TypeScript cho thuộc tính (thường là camelCase nếu đó là một từ đơn tự nhiên, ví dụ `color`, `size`). Tuy nhiên, BEARD cấm `camelCase` đa từ. Do đó, nếu tên là một từ đơn, ví dụ `color`, thì vẫn hợp lệ. Nếu là `backgroundColor`, nó sẽ bị cấm. Trong trường hợp này, bạn cần tìm một từ đơn như `background` hoặc `fill`.
      * Ví dụ (TypeScript):
        ```typescript
        class Shape {
            id: Identifier;
            position: Point; // Point, Identifier là các type một từ
            fill: Color;     // Color là type một từ
        }
        ```

  * **`parameters` (Tham số hàm/phương thức):**

      * Mô tả: Tên tham số truyền vào hàm hoặc phương thức.
      * Quy tắc: Một từ đơn, có nghĩa, thường là danh từ. Viết thường (lowercase).
      * Ví dụ (TypeScript):
        ```typescript
        function move(object: Node, distance: Vector): void {
            // ...
        }
        ```

  * **`[abstracts]able` / `[interfaces]able` (Interfaces định nghĩa hành vi):**

      * Mô tả: Interfaces trong TypeScript định nghĩa một tập hợp các phương thức hoặc thuộc tính mà một class có thể implement (định nghĩa hành vi hoặc khả năng).
      * Quy tắc BEARD (Rule 2): `[TừĐơn]able` hoặc `[TừĐơn]ible`.
      * Ví dụ (TypeScript):
        ```typescript
        interface Renderable {
            render(context: GPUCanvasContext): void;
        }

        interface Serializable {
            serialize(): ArrayBuffer;
        }

        interface Updatable {
            update(delta: Timespan): void; // Timespan là type một từ
        }
        ```
      * Lưu ý: Interfaces chỉ dùng để định nghĩa cấu trúc dữ liệu (data shapes) không bắt buộc theo hậu tố `able/ible` và có thể dùng một từ đơn (ví dụ: `interface Point { x: number; y: number; }`).

  * **`classes` (Lớp):**

      * Mô tả: Khai báo lớp trong TypeScript.
      * Quy tắc: Một từ đơn, có nghĩa, danh từ, viết hoa chữ cái đầu (PascalCase cho một từ).
      * Ví dụ (TypeScript):
        ```typescript
        class Document { /* ... */ }
        class Renderer { /* ... */ }
        class Parser { /* ... */ }
        ```

  * **`constants` (Hằng số):**

      * Mô tả: Các giá trị không thay đổi.
      * Quy tắc: Một từ đơn, có nghĩa. Viết hoa chữ cái đầu (PascalCase cho một từ) để dễ nhận biết. Nếu khái niệm là đa từ không thể rút gọn thành một từ PascalCase duy nhất mà không vi phạm regex cấm đa từ (ví dụ `DefaultBackgroundColor` sẽ bị cấm), hãy sử dụng một từ đơn viết thường (ví dụ `defaultbackground`) hoặc chia nhỏ cấu trúc (ví dụ `Theme.Background`).
      * Ví dụ (TypeScript):
        ```typescript
        export const Version = "1.0.0";
        export const Pi = 3.14159;
        export const Defaultradius = 10.0; // "Defaultradius" là một từ, "radius" viết thường
        export const Maximumvalue = 1000;  // "Maximumvalue" là một từ, "value" viết thường
        ```

  * **`type associated generics` (Type Aliases và Generics):**

      * Mô tả: Định danh cho kiểu dữ liệu tùy chỉnh (type aliases) hoặc tham số generic.
      * Quy tắc (Type Aliases): Một từ đơn, có nghĩa, viết hoa chữ cái đầu (PascalCase cho một từ).
      * Quy tắc (Generic Parameters): Thường là một chữ cái viết hoa theo quy ước (T, U, K, V, E), hoặc một từ đơn PascalCase nếu cần mô tả rõ hơn và là một phần của định nghĩa kiểu do bạn tạo ra.
      * Ví dụ (TypeScript):
        ```typescript
        type Identifier = string;
        type Coordinate = number;
        type Points = Array<Point>; // Point là một type khác

        interface Container<Element> { // Element là generic parameter một từ
            add(item: Element): void;
            get(id: Identifier): Element | undefined;
        }

        type Callback<Argument> = (data: Argument) => void; // Argument là generic parameter
        ```

  * **`functions` (Hàm):**

      * Mô tả: Tên hàm hoặc phương thức.
      * Quy tắc: Một từ đơn, có nghĩa, thường là động từ hoặc cụm động từ rút gọn thành một từ. Viết thường (lowercase).
      * Ví dụ (TypeScript):
        ```typescript
        function initialize(): Promise<void> { /* ... */ }
        function parse(source: string): Document { /* ... */ }
        function tessellate(path: PathData): VertexData { /* ... */ } // PathData, VertexData là type
        ```

**3. Các Cân nhắc Kiến trúc Quan trọng:**

  * **Zero-Copy Data Handling (WGPU và Workers):**

      * **Mục tiêu:** Giảm thiểu hoặc loại bỏ việc sao chép dữ liệu không cần thiết giữa CPU và GPU, cũng như giữa luồng chính và Web Workers.
      * **WGPU:** WGPU được thiết kế để làm việc hiệu quả với `ArrayBuffer`. Dữ liệu hình học (vertices, indices) nên được chuẩn bị trong `Float32Array`, `Uint32Array`, v.v., được cấp phát từ `ArrayBuffer` và ghi trực tiếp vào `GPUBuffer`.
      * **Web Workers:**
          * Sử dụng `SharedArrayBuffer` để cho phép nhiều luồng (luồng chính và workers) truy cập cùng một vùng nhớ. Điều này đòi hỏi cơ chế đồng bộ hóa cẩn thận bằng `Atomics` (ví dụ: `Atomics.wait`, `Atomics.notify`, `Atomics.store`, `Atomics.compareExchange`).
          * Khi truyền dữ liệu lớn cho worker mà không cần chia sẻ trạng thái liên tục, hãy truyền `ArrayBuffer` dưới dạng `Transferable Object` qua `postMessage()`. Quyền sở hữu buffer sẽ được chuyển giao, tránh sao chép.
      * **SVG Parsing:** Quá trình phân tích SVG nên tạo ra các cấu trúc dữ liệu (ví dụ, danh sách các điểm cho một path) có thể dễ dàng chuyển đổi hoặc ánh xạ trực tiếp vào các `ArrayBuffer` để gửi đến WGPU hoặc workers.

  * **An toàn Luồng với Web Workers:**

      * **Tách biệt tác vụ:** Offload các tác vụ tính toán nặng (ví dụ: tessellation phức tạp của path SVG, phân tích style phức tạp, các thuật toán hình học) sang Web Workers để không làm block luồng chính (UI thread).
      * **Giao tiếp:**
          * `SharedArrayBuffer` + `Atomics`: Mạnh mẽ cho các cấu trúc dữ liệu dùng chung, nhưng phức tạp hơn trong việc quản lý đồng bộ hóa để tránh race conditions.
          * `postMessage` (với Transferable Objects cho zero-copy khi có thể): Đơn giản hơn cho các tác vụ "fire-and-forget" hoặc khi kết quả không cần cập nhật liên tục vào một vùng nhớ dùng chung.
      * **WGPU Context:** Hầu hết các tương tác với `GPUDevice` và `GPUQueue` phải được thực hiện trên luồng chính nơi context WGPU được tạo. Workers có thể chuẩn bị dữ liệu, nhưng lệnh gọi WGPU thực tế thường được gửi từ luồng chính. Các API mới hơn như OffscreenCanvas có thể cung cấp một số linh hoạt, nhưng cần kiểm tra kỹ hỗ trợ và tính phức tạp.
      * **Trạng thái:** Quản lý cẩn thận trạng thái nào thuộc về luồng chính và trạng thái nào thuộc về worker. Tránh sửa đổi đồng thời dữ liệu chia sẻ mà không có cơ chế khóa hoặc đồng bộ hóa phù hợp.


tuân thủ kiến trúc tiến hóa, khả năng bảo trì, và sự phân tách rõ ràng trách nhiệm giữa các module, đồng thời đảm bảo hiệu suất cao và các yêu cầu kỹ thuật bạn đã đề ra.

**1. Cấu trúc Thư mục Đề xuất (Cập nhật với `index.ts`):**

Các tệp `index.ts` được thêm vào để quản lý việc export các thành phần public của mỗi module, giúp import gọn gàng hơn ở các module khác.

```
src/
├── core/                    # Tương tác WGPU, pipeline render, tiện ích đồ họa cấp thấp
│   ├── engine/              # Abstraction chính của WGPU (device, queue, swapchain)
│   │   └── index.ts
│   ├── buffer/              # Quản lý GPU buffer, tiện ích zero-copy
│   │   └── index.ts
│   ├── shader/              # Tải và quản lý shader (WGSL)
│   │   └── index.ts
│   ├── pipeline/            # Thiết lập render pipeline
│   │   └── index.ts
│   ├── texture/             # Xử lý texture
│   │   └── index.ts
│   ├── resource/            # Các tài nguyên GPU khác (sampler, bind group layout)
│   │   └── index.ts
│   └── index.ts             # Export từ các submodule của core
│
├── svg/                     # Phân tích, biểu diễn và xử lý SVG
│   ├── dom/                 # Biểu diễn cấu trúc SVG (tương tự DOM)
│   │   └── index.ts
│   ├── parser/              # Phân tích chuỗi SVG thành cấu trúc DOM nội bộ
│   │   └── index.ts
│   ├── path/                # Phân tích và xử lý dữ liệu path SVG
│   │   └── index.ts
│   ├── style/               # Phân tích và áp dụng style SVG
│   │   └── index.ts
│   ├── geometry/            # Tính toán hình học, tessellation cho rendering
│   │   └── index.ts
│   └── index.ts             # Export từ các submodule của svg
│
├── editor/                  # Logic đặc thù của editor, trạng thái UI, công cụ
│   ├── state/               # Quản lý trạng thái ứng dụng, lịch sử, vùng chọn
│   │   └── index.ts
│   ├── tool/                # Các công cụ chỉnh sửa (chọn, vẽ đường, hình khối)
│   │   └── index.ts
│   ├── command/             # Command pattern cho undo/redo
│   │   └── index.ts
│   ├── view/                # Quản lý viewport (zoom, pan)
│   │   └── index.ts
│   └── index.ts             # Export từ các submodule của editor
│
├── worker/                  # Mã nguồn liên quan đến Web Worker
│   ├── manager/             # Quản lý worker pool, phân phối tác vụ
│   │   └── index.ts
│   ├── task/                # Định nghĩa các tác vụ offload cho worker
│   │   └── index.ts
│   ├── serialize/           # (De)serialization cho message worker (tập trung zero-copy)
│   │   └── index.ts
│   └── index.ts             # Export từ các submodule của worker
│
├── platform/                # Tích hợp đặc thù của trình duyệt
│   ├── input/               # Xử lý sự kiện chuột, bàn phím
│   │   └── index.ts
│   ├── file/                # Truy cập file hệ thống (nếu có)
│   │   └── index.ts
│   └── index.ts
│
├── ui/                      # Các thành phần giao diện người dùng (nếu không dùng framework ngoài)
│   ├── atom/                # Các phần tử UI cơ bản (button, input)
│   │   └── index.ts
│   ├── molecule/            # (Tương tự Atomic Design)
│   │   └── index.ts
│   ├── organism/            # (Tương tự Atomic Design)
│   │   └── index.ts
│   └── index.ts
│
└── common/                  # Tiện ích, kiểu dữ liệu, hằng số dùng chung
    ├── type/                # Các TypeScript type, interface (chỉ định nghĩa cấu trúc dữ liệu)
    │   └── index.ts
    ├── constant/            # Hằng số toàn cục
    │   └── index.ts
    ├── math/                # Thư viện toán học (vector, matrix - nếu tự viết)
    │   └── index.ts
    ├── error/               # Các kiểu lỗi tùy chỉnh
    │   └── index.ts
    └── index.ts             # Export từ các submodule của common
```

  * **Vai trò của `index.ts`:** Mỗi tệp `index.ts` trong một thư mục module sẽ export các thành phần (classes, functions, types, constants) mà module đó muốn cung cấp ra bên ngoài. Điều này cho phép import ngắn gọn hơn, ví dụ: `import { Device, Queue } from 'core/engine';` thay vì chỉ đường dẫn đến tệp cụ thể.

**2. Sự Phụ thuộc giữa các Submodule (Dependency Flow):**

Mặc dù không có biểu đồ phụ thuộc cứng nhắc ở giai đoạn này, luồng phụ thuộc điển hình có thể được hình dung như sau:

  * **`core`**: Là module cấp thấp nhất, không phụ thuộc vào các module nghiệp vụ khác như `svg` hay `editor`. Nó cung cấp các abstraction cho WGPU.
  * **`common`**: Chứa các tiện ích, kiểu dữ liệu cơ bản, hằng số có thể được sử dụng bởi tất cả các module khác. Nó nên có ít hoặc không có phụ thuộc vào các module nghiệp vụ.
  * **`svg`**: Phụ thuộc vào `common` (cho kiểu dữ liệu, toán học) và có thể là `core` nếu có các tác vụ liên quan đến GPU (ví dụ, chuẩn bị dữ liệu cho `core/buffer` từ hình học SVG).
  * **`worker`**: Có thể phụ thuộc vào `common` và `svg` (ví dụ, một `task` trong worker có thể cần phân tích hoặc xử lý dữ liệu SVG). Logic `serialize` có thể cần các kiểu từ `common`.
  * **`editor`**: Là module cấp cao, phụ thuộc vào `core` (để render), `svg` (để làm việc với đối tượng SVG), `worker` (để offload task), `common`, và `platform`.
  * **`platform`**: Cung cấp các API của trình duyệt, có thể được sử dụng bởi `editor` hoặc các module khác cần tương tác với môi trường.
  * **`ui`**: Nếu được phát triển, sẽ phụ thuộc vào `common` và có thể tương tác với `editor` để hiển thị và cập nhật trạng thái.

**3. Quy ước Đặt tên (Theo BEARD và Yêu cầu - Không đổi):**

Quy ước đặt tên vẫn giữ nguyên như đã trình bày ở câu trả lời trước, nhấn mạnh "một từ đơn, đầy đủ ý nghĩa" và các quy tắc BEARD cụ thể cho từng loại định danh.

  * **`fields`**: `id`, `position`, `fill`
  * **`parameters`**: `object`, `distance`
  * **`[abstracts]able` / `[interfaces]able`**: `Renderable`, `Serializable`, `Updatable`
      * Interface dữ liệu: `Point`, `Color`
  * **`classes`**: `Document`, `Renderer`, `Parser`
  * **`constants`**: `Version`, `Pi`, `Defaultradius`, `Maximumvalue`
  * **`type associated generics` (Type Aliases & Generics)**: `Identifier`, `Coordinate`, `Container<Element>`, `Callback<Argument>`
  * **`functions`**: `initialize`, `parse`, `tessellate`

**4. Danh sách Đơn từ Gợi ý cho Định danh (Theo Submodule):**

Dưới đây là danh sách các "đơn từ" (single-word identifiers) gợi ý cho việc đặt tên, tuân thủ quy tắc BEARD. Các từ này có thể được dùng làm tên class, type (PascalCase), hàm, biến (lowercase), hoặc hằng số (PascalCase nếu là một khái niệm đơn lẻ).

Lưu ý: Việc tạo ra 100+ *hoàn toàn độc đáo và phù hợp* cho *mỗi* sub-submodule là rất khó. Danh sách này tập trung vào các module chính và cung cấp một nền tảng tốt. Bạn có thể kết hợp hoặc tìm từ đồng nghĩa khi cần.

**`src/core/` (Tổng quan cho WGPU và Rendering)**

  * Nouns (Types/Classes/Constants): `Core`, `Graphics`, `Engine`, `Renderer`, `Device`, `Adapter`, `Instance`, `Surface`, `Queue`, `Context`, `Pipeline`, `Shader`, `Buffer`, `Texture`, `Sampler`, `Resource`, `Binding`, `Layout`, `Group`, `Stage`, `Pass`, `Frame`, `Viewport`, `Scissor`, `Target`, `Format`, `Limit`, `Feature`, `Extension`, `Capability`, `State`, `Descriptor`, `Configuration`, `Setup`, `Control`, `Manager`, `Handler`, `System`, `Interface`, `Abstraction`, `Wrapper`, `Utility`, `Helper`, `Kernel`, `Display`, `Window`, `Screen`, `Swapchain`, `Framebuffer`, `Depthstencil`, `Multisample`, `Query`, `Timestamp`, `Fence`, `Event`, `Signal`, `Command`, `Encoder`, `Bundle`, `Memory`, `Allocation`, `Pool`, `Heap`, `Access`, `Usage`, `Flag`, `Mode`, `Type`, `Kind`, `Identifier`, `Handle`, `Reference`, `Pointer`, `Offset`, `Size`, `Extent`, `Dimension`, `Origin`, `Color`, `Depth`, `Stencil`, `Blend`, `Raster`, `Vertex`, `Fragment`, `Compute`, `Tessellation`, `Mesh`, `Primitive`, `Topology`, `Index`, `Attribute`, `Input`, `Output`, `Constant`, `Uniform`, `Storage`, `Workgroup`, `Invocation`, `Barrier`, `Synchronization`, `Performance`, `Metric`, `Log`, `Trace`, `Debug`, `Profile`, `Capture`, `Replay`
  * Verbs (Functions): `initialize`, `create`, `destroy`, `begin`, `end`, `submit`, `present`, `draw`, `dispatch`, `bind`, `set`, `get`, `update`, `configure`, `compile`, `link`, `attach`, `detach`, `map`, `unmap`, `read`, `write`, `copy`, `clear`, `resolve`, `execute`, `wait`, `poll`, `query`, `enable`, `disable`, `toggle`, `reset`, `flush`, `release`, `acquire`, `validate`, `inspect`, `monitor`

**`src/core/engine/` (WGPU Abstraction)**

  * Nouns: `Device`, `Adapter`, `Queue`, `Instance`, `Surface`, `Configuration`, `Request`, `Feature`, `Limit`, `Callback`, `Handler`, `Future`, `Promise`, `State`, `Status`, `Error`, `Creation`, `Destruction`, `Context`, `Session`, `Mainthread`, `Offscreencanvas`
  * Verbs: `request`, `select`, `create`, `connect`, `disconnect`, `poll`, `configure`, `lose`, `restore`, `check`, `enumerate`, `listen`

**`src/core/buffer/` (GPU Buffer Management)**

  * Nouns: `Buffer`, `Allocation`, `View`, `Slice`, `Range`, `Offset`, `Size`, `Capacity`, `Usage`, `Access`, `Mapping`, `Pointer`, `Data`, `Content`, `Update`, `Copy`, `Transfer`, `Staging`, `Vertexbuffer`, `Indexbuffer`, `Uniformbuffer`, `Storagebuffer`, `Readback`, `Upload`, `Download`, `Descriptor`, `State`, `Memory`, `Block`, `Chunk`, `Pool`, `Strategy`
  * Verbs: `create`, `destroy`, `map`, `unmap`, `write`, `read`, `copy`, `update`, `clear`, `resize`, `allocate`, `free`, `bind`

**`src/core/shader/` (Shader Loading & Management)**

  * Nouns: `Shader`, `Module`, `Source`, `Code`, `Binary`, `SPIRV`, `WGSL`, `Compilation`, `Linking`, `Pipeline`, `Stage`, `Vertex`, `Fragment`, `Compute`, `EntryPoint`, `Interface`, `Binding`, `Resource`, `Constant`, `Specialization`, `Reflection`, `Metadata`, `Error`, `Warning`, `Log`, `Cache`, `Manager`, `Loader`, `Preprocessor`, `Include`, `Define`
  * Verbs: `compile`, `load`, `parse`, `validate`, `reflect`, `link`, `create`, `get`, `resolve`, `preprocess`, `cache`, `clear`

**`src/core/pipeline/` (Render Pipeline Setup)**

  * Nouns: `Pipeline`, `Renderpipeline`, `Computepipeline`, `Layout`, `Descriptor`, `State`, `Vertex`, `Fragment`, `Compute`, `Target`, `Blend`, `Depthstencil`, `Multisample`, `Primitive`, `Rasterization`, `Shader`, `Module`, `EntryPoint`, `Bufferlayout`, `Attributelayout`, `Bindinglayout`, `Group`, `Cache`, `Creation`, `Configuration`
  * Verbs: `create`, `configure`, `build`, `compile`, `get`, `cache`, `reuse`, `bind`

**`src/core/texture/` (Texture Handling)**

  * Nouns: `Texture`, `View`, `Sampler`, `Format`, `Dimension`, `Kind`, `Size`, `Mipmap`, `Layer`, `Array`, `Cubemap`, `Volume`, `Surface`, `Pixel`, `Texel`, `Data`, `Source`, `Upload`, `Copy`, `Transfer`, `Usage`, `Access`, `Descriptor`, `State`, `Allocation`, `Resource`, `Image`, `Framebufferattachment`
  * Verbs: `create`, `destroy`, `write`, `read`, `copy`, `generate`, `sample`, `bind`, `configure`, `resize`

**`src/core/resource/` (General GPU Resources)**

  * Nouns: `Resource`, `Handle`, `Identifier`, `Type`, `Kind`, `Lifetime`, `Scope`, `Manager`, `Pool`, `Cache`, `Tracker`, `Dependency`, `State`, `Transition`, `Barrier`, `Bindgroup`, `Bindlayout`, `Descriptor`, `Set`, `Entry`, `Visibility`, `Queryset`, `Bundle`, `Encoder`
  * Verbs: `create`, `destroy`, `allocate`, `free`, `track`, `manage`, `bind`, `access`, `reference`, `release`

**`src/svg/` (Tổng quan cho SVG)**

  * Nouns: `SVG`, `Document`, `Element`, `Node`, `Attribute`, `Style`, `Shape`, `Path`, `Text`, `Image`, `Group`, `Symbol`, `Definition`, `Use`, `Pattern`, `Gradient`, `Filter`, `Mask`, `Clip`, `Marker`, `Viewbox`, `Transform`, `Coordinate`, `Unit`, `Color`, `Opacity`, `Stroke`, `Fill`, `Font`, `Metadata`, `DOM`, `Parser`, `Serializer`, `Renderer`, `Interactor`, `Validator`, `Optimizer`, `Converter`, `Geometry`, `Bounds`, `Matrix`, `Vector`, `Point`, `Size`, `Rectangle`, `Circle`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Curve`, `Arc`, `Segment`, `Command`, `Property`, `Value`, `Selector`, `Rule`, `Stylesheet`, `Cascade`, `Inheritance`, `Layout`, `Animation`, `Event`, `Script`, `Structure`, `Tree`, `Model`, `Representation`, `Abstract`, `Concrete`, `Instance`, `Prototype`, `Factory`, `Builder`, `Visitor`, `Context`, `State`, `Error`, `Warning`, `Log`, `Performance`
  * Verbs: `parse`, `serialize`, `render`, `create`, `add`, `remove`, `get`, `set`, `update`, `transform`, `query`, `select`, `calculate`, `measure`, `intersect`, `contain`, `optimize`, `validate`, `convert`, `animate`, `handle`, `traverse`, `walk`, `build`, `apply`, `resolve`, `inherit`, `draw`, `paint`, `clip`, `mask`

**`src/svg/dom/` (SVG DOM Representation)**

  * Nouns: `Element`, `Node`, `Attribute`, `Textnode`, `Parent`, `Child`, `Sibling`, `Root`, `Tree`, `Structure`, `Collection`, `List`, `Map`, `Query`, `Selector`, `Path`, `Shape`, `Group`, `Text`, `Image`, `Symbol`, `Definition`, `Metadata`, `Style`, `Script`, `Unknown`, `Namespace`, `Prefix`, `Tagname`, `Identifier`, `Reference`, `Link`, `Hierarchy`, `Interface`
  * Verbs: `create`, `append`, `insert`, `remove`, `replace`, `get`, `set`, `query`, `find`, `traverse`, `clone`, `adopt`, `normalize`

**`src/svg/parser/` (SVG String to DOM)**

  * Nouns: `Parser`, `Tokenizer`, `Lexer`, `Stream`, `Buffer`, `Char`, `Token`, `Rule`, `Grammar`, `State`, `Context`, `Handler`, `Builder`, `Stack`, `Error`, `Recovery`, `Position`, `Location`, `Source`, `Input`, `Mode`, `Entity`, `Comment`, `Doctype`, `CDATA`, `Instruction`, `Progress`, `Result`
  * Verbs: `parse`, `tokenize`, `lex`, `consume`, `peek`, `next`, `push`, `pop`, `build`, `handle`, `error`, `recover`, `skip`, `read`

**`src/svg/path/` (Path Data Parsing & Manipulation)**

  * Nouns: `Path`, `Segment`, `Command`, `Move`, `Line`, `Curve`, `Arc`, `Close`, `Point`, `Coordinate`, `Vector`, `Parameter`, `Data`, `Parser`, `Serializer`, `Builder`, `Iterator`, `Transformer`, `Simplifier`, `Normalizer`, `Length`, `Area`, `Centroid`, `Pointatlength`, `Tangent`, `Normal`, `Boundingbox`
  * Verbs: `parse`, `serialize`, `add`, `get`, `transform`, `simplify`, `normalize`, `calculate`, `offset`, `interpolate`, `approximate`, `convert`

**`src/svg/style/` (Style Parsing & Application)**

  * Nouns: `Style`, `Property`, `Value`, `Unit`, `Selector`, `Rule`, `Stylesheet`, `Cascade`, `Inheritance`, `Specificity`, `Computedvalue`, `Specifiedvalue`, `Initialvalue`, `Attribute`, `Presentation`, `Inline`, `External`, `Parser`, `Resolver`, `Matcher`, `Applier`, `Context`, `Element`, `Declaration`, `Block`, `Importance`, `Color`, `Font`, `Text`, `Layout`, `Filtereffect`
  * Verbs: `parse`, `apply`, `resolve`, `compute`, `match`, `get`, `set`, `inherit`, `cascade`, `validate`

**`src/svg/geometry/` (Geometric Calculations, Tessellation)**

  * Nouns: `Geometry`, `Shape`, `Point`, `Vector`, `Matrix`, `Transform`, `Rectangle`, `Circle`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Curve`, `Arc`, `Bounds`, `Intersection`, `Union`, `Difference`, `Area`, `Perimeter`, `Centroid`, `Distance`, `Angle`, `Tessellation`, `Triangulation`, `Vertex`, `Index`, `Normal`, `Mesh`, `Algorithm`, `Precision`, `Tolerance`, `Epsilon`, `Projection`, `Decomposition`
  * Verbs: `calculate`, `transform`, `intersect`, `contain`, `measure`, `tessellate`, `triangulate`, `simplify`, `offset`, `rotate`, `scale`, `translate`, `project`, `decompose`, `normalize`

**`src/editor/` (Tổng quan cho Editor Logic)**

  * Nouns: `Editor`, `Document`, `View`, `Canvas`, `Tool`, `Selection`, `History`, `Command`, `Action`, `State`, `Mode`, `Context`, `Session`, `User`, `Input`, `Event`, `Handler`, `Manager`, `Controller`, `Model`, `Viewmodel`, `Project`, `File`, `Asset`, `Layer`, `Object`, `Property`, `Inspector`, `Palette`, `Panel`, `Workspace`, `Settings`, `Preferences`, `Configuration`, `Plugin`, `Extension`, `Scripting`, `Automation`, `Macro`, `Undo`, `Redo`, `Clipboard`, `Copy`, `Paste`, `Cut`, `Delete`, `Drag`, `Drop`, `Resize`, `Rotate`, `Scale`, `Skew`, `Align`, `Distribute`, `Group`, `Ungroup`, `Combine`, `Breakapart`, `Zoom`, `Pan`, `Grid`, `Snap`, `Ruler`, `Guide`, `Export`, `Import`, `Save`, `Load`, `Render`, `Preview`, `Feedback`, `Notification`, `Progress`, `Error`, `Helper`, `Utility`, `Interface`
  * Verbs: `open`, `close`, `save`, `load`, `create`, `delete`, `select`, `deselect`, `edit`, `update`, `apply`, `execute`, `undo`, `redo`, `copy`, `paste`, `cut`, `draw`, `move`, `resize`, `rotate`, `zoom`, `pan`, `snap`, `align`, `group`, `ungroup`, `render`, `export`, `import`, `handle`, `dispatch`, `notify`, `configure`, `register`, `activate`, `deactivate`

**`src/editor/state/` (Application State Management)**

  * Nouns: `State`, `Store`, `Document`, `Selection`, `History`, `Stack`, `Mutation`, `Action`, `Reducer`, `Effect`, `Subscription`, `Snapshot`, `Patch`, `Diff`, `Current`, `Previous`, `Next`, `Transaction`, `Scope`, `Context`, `Model`, `Data`, `Tree`, `Change`, `Event`, `Log`, `Persistence`, `Hydration`, `Rehydration`, `Atom`, `Selector`
  * Verbs: `dispatch`, `commit`, `select`, `subscribe`, `get`, `set`, `update`, `reset`, `undo`, `redo`, `push`, `pop`, `apply`, `revert`, `save`, `load`, `hydrate`, `monitor`

**`src/editor/tool/` (Editing Tools)**

  * Nouns: `Tool`, `Selector`, `Pointer`, `Pen`, `Pencil`, `Brush`, `Eraser`, `Line`, `Rectangle`, `Ellipse`, `Polygon`, `Star`, `Text`, `Path`, `Nodeeditor`, `Gradienteditor`, `Eyedropper`, `Zoom`, `Pan`, `Hand`, `Measure`, `Transform`, `Rotate`, `Scale`, `Skew`, `Aligner`, `Distributor`, `Shaper`, `Cutter`, `Joiner`, `Mode`, `State`, `Handler`, `Interactor`, `Feedback`, `Cursor`, `Icon`, `Property`, `Option`, `Setting`, `Manager`, `Registry`, `Active`, `Default`
  * Verbs: `activate`, `deactivate`, `select`, `create`, `draw`, `edit`, `modify`, `handle`, `click`, `drag`, `move`, `release`, `hover`, `keydown`, `keyup`, `configure`, `reset`, `apply`

**`src/editor/command/` (Command Pattern for Undo/Redo)**

  * Nouns: `Command`, `Action`, `Operation`, `Transaction`, `Instruction`, `Step`, `Macro`, `History`, `Stack`, `Executor`, `Invoker`, `Receiver`, `Parameter`, `Argument`, `State`, `Memento`, `Undoable`, `Redoable`, `Composite`, `Sequence`, `Factory`, `Registry`, `Type`, `Name`, `Label`, `Description`, `Result`, `Effect`, `Change`
  * Verbs: `execute`, `undo`, `redo`, `apply`, `revert`, `create`, `record`, `commit`, `serialize`, `deserialize`, `combine`, `group`

**`src/editor/view/` (Viewport Management)**

  * Nouns: `View`, `Viewport`, `Canvas`, `Renderer`, `Camera`, `Projection`, `Transform`, `Matrix`, `Zoom`, `Pan`, `Scroll`, `Offset`, `Scale`, `Center`, `Bounds`, `Frame`, `Grid`, `Ruler`, `Guide`, `Overlay`, `Layer`, `Background`, `Foreground`, `Cursor`, `Crosshair`, `Feedback`, `Interaction`, `Handler`, `State`, `Update`, `Display`, `Refresh`, `Culling`, `Optimization`, `Pixel`, `Coordinate`, `Screen`, `World`, `Model`
  * Verbs: `render`, `update`, `zoom`, `pan`, `scroll`, `set`, `get`, `transform`, `project`, `unproject`, `fit`, `center`, `show`, `hide`, `toggle`, `refresh`, `draw`, `invalidate`

**`src/worker/` (Tổng quan cho Web Workers)**

  * Nouns: `Worker`, `Thread`, `Task`, `Job`, `Process`, `Pool`, `Manager`, `Dispatcher`, `Scheduler`, `Queue`, `Message`, `Payload`, `Data`, `Buffer`, `Sharedbuffer`, `Arraybuffer`, `Atom`, `Lock`, `Mutex`, `Semaphore`, `Channel`, `Port`, `Communication`, `Protocol`, `Serialization`, `Deserialization`, `Script`, `Module`, `Context`, `Scope`, `Lifecycle`, `State`, `Error`, `Result`, `Promise`, `Future`, `Progress`, `Cancellation`, `Timeout`, `Priority`, `Strategy`, `Configuration`, `Interface`, `Offload`, `Compute`, `Parallel`, `Concurrent`
  * Verbs: `create`, `spawn`, `terminate`, `post`, `send`, `receive`, `handle`, `execute`, `run`, `dispatch`, `schedule`, `cancel`, `await`, `notify`, `synchronize`, `serialize`, `deserialize`, `manage`, `monitor`, `log`

**`src/worker/manager/` (Worker Pool Management)**

  * Nouns: `Manager`, `Pool`, `Worker`, `Instance`, `Fleet`, `Farm`, `Controller`, `Strategy`, `Allocator`, `Balancer`, `Router`, `Queue`, `Task`, `Job`, `Lifecycle`, `State`, `Availability`, `Capacity`, `Load`, `Metric`, `Health`, `Configuration`, `Policy`, `Supervisor`
  * Verbs: `create`, `destroy`, `add`, `remove`, `acquire`, `release`, `dispatch`, `assign`, `manage`, `monitor`, `balance`, `scale`, `update`

**`src/worker/task/` (Task Definitions for Workers)**

  * Nouns: `Task`, `Job`, `Unit`, `Work`, `Operation`, `Definition`, `Type`, `Kind`, `Identifier`, `Input`, `Output`, `Parameter`, `Argument`, `Data`, `Payload`, `Result`, `Error`, `State`, `Status`, `Progress`, `Priority`, `Cancellationtoken`, `Context`, `Script`, `Function`, `Callable`, `Runnable`, `Descriptor`
  * Verbs: `define`, `create`, `execute`, `run`, `process`, `cancel`, `complete`, `fail`, `report`, `update`

**`src/worker/serialize/` (Serialization/Deserialization for Worker Messages)**

  * Nouns: `Serializer`, `Deserializer`, `Format`, `Protocol`, `Schema`, `Message`, `Payload`, `Data`, `Buffer`, `Stream`, `Encoder`, `Decoder`, `Strategy`, `Type`, `Registry`, `Version`, `Compatibility`, `ZeroCopy`, `Transferable`, `JSON`, `Binary`, `Custom`
  * Verbs: `serialize`, `deserialize`, `encode`, `decode`, `pack`, `unpack`, `convert`, `transform`, `register`

**`src/common/` (Tổng quan cho Tiện ích Chung)**

  * Nouns: `Utility`, `Helper`, `Service`, `Constant`, `Type`, `Interface`, `Enum`, `Struct`, `Function`, `Math`, `Vector`, `Matrix`, `Point`, `Size`, `Rectangle`, `Color`, `String`, `Number`, `Boolean`, `Date`, `Time`, `Duration`, `Interval`, `Regex`, `Pattern`, `Error`, `Exception`, `Result`, `Option`, `Maybe`, `Either`, `Validation`, `Parser`, `Formatter`, `Converter`, `Generator`, `Iterator`, `Collection`, `List`, `Map`, `Set`, `Queue`, `Stack`, `Tree`, `Graph`, `Node`, `Edge`, `Event`, `Signal`, `Dispatcher`, `Listener`, `Observer`, `Subject`, `Promise`, `Future`, `Async`, `Await`, `Logger`, `Tracer`, `Monitor`, `Config`, `Setting`, `Environment`, `Version`, `Identifier`, `UUID`, `GUID`, `Hash`, `Checksum`, `Sort`, `Search`, `Algorithm`, `Cache`, `Memoization`, `Debounce`, `Throttle`, `Retry`, `Timeout`, `I18N`, `L10N`, `Format`, `Locale`, `Timezone`, `Platform`, `OS`, `Browser`, `Device`, `Network`, `URL`, `URI`, `Path`, `File`, `Stream`, `Buffer`, `Encoding`, `Decoding`, `Crypto`, `Secure`, `Random`, `Permission`, `Role`, `User`, `Session`, `Token`, `State`, `Context`, `Decorator`, `Mixin`, `Plugin`, `Module`, `Library`, `Framework`, `Application`, `System`
  * Verbs: `create`, `get`, `set`, `is`, `as`, `to`, `from`, `parse`, `format`, `validate`, `convert`, `generate`, `calculate`, `measure`, `compare`, `equals`, `sort`, `search`, `find`, `filter`, `map`, `reduce`, `each`, `for`, `while`, `if`, `switch`, `try`, `catch`, `throw`, `log`, `trace`, `debug`, `warn`, `error`, `assert`, `check`, `verify`, `ensure`, `resolve`, `reject`, `await`, `sleep`, `debounce`, `throttle`, `retry`, `timeout`, `load`, `save`, `read`, `write`, `copy`, `move`, `delete`, `list`, `exists`, `watch`, `connect`, `disconnect`, `send`, `receive`, `Workspace`, `request`, `normalize`, `sanitize`, `escape`, `unescape`, `encode`, `decode`, `encrypt`, `decrypt`, `hash`, `sign`, `verify`, `random`, `clamp`, `lerp`, `smooth`, `round`, `floor`, `ceil`, `abs`, `min`, `max`, `pow`, `sqrt`, `sin`, `cos`, `tan`

**5. Các Cân nhắc Kiến trúc Quan trọng (Không đổi):**
Các cân nhắc về Zero-Copy Data Handling và An toàn Luồng với Web Workers vẫn giữ nguyên tầm quan trọng như đã mô tả trước đó.

Hy vọng rằng phiên bản cập nhật và chi tiết này, đặc biệt với các danh sách đơn từ gợi ý, sẽ giúp bạn đẩy nhanh quá trình phát triển dự án SVG editor.

-----

**Nội dung các tệp quản lý trạng thái:**

**`architecture.md`:**

```markdown
# Project: SVG Editor (WGPU, TypeScript, Zero-Copy, Workers)

## 1. Initial Goal

Design a foundational directory structure and key component types for a high-performance SVG editor using TypeScript and WGPU, emphasizing zero-copy data handling, thread safety with Web Workers, and minimal external libraries, adhering to BEARD naming conventions.

## 2. Architectural Philosophy

- **BEARD (Atomic Design FE adapted for structure, Evolutionary Backend adapted for workers/core logic):** Modules are clearly defined with specific responsibilities. Worker logic is designed for independent evolution.
- **WGPU for Rendering:** Leverage WGPU for high-performance graphics.
- **Zero-Copy:** Prioritize efficient data pathways.
- **Thread Safety:** Utilize Web Workers for parallelism with careful synchronization.

## 3. Core Technologies

- TypeScript
- WGPU (WebGPU API)
- Web Workers

## 4. Proposed Directory Structure (`src/`) (Updated with `index.ts`)

```

src/
├── core/                    \# Tương tác WGPU, pipeline render, tiện ích đồ họa cấp thấp
│   ├── engine/              \# Abstraction chính của WGPU (device, queue, swapchain)
│   │   └── index.ts
│   ├── buffer/              \# Quản lý GPU buffer, tiện ích zero-copy
│   │   └── index.ts
│   ├── shader/              \# Tải và quản lý shader (WGSL)
│   │   └── index.ts
│   ├── pipeline/            \# Thiết lập render pipeline
│   │   └── index.ts
│   ├── texture/             \# Xử lý texture
│   │   └── index.ts
│   ├── resource/            \# Các tài nguyên GPU khác (sampler, bind group layout)
│   │   └── index.ts
│   └── index.ts             \# Export từ các submodule của core
│
├── svg/                     \# Phân tích, biểu diễn và xử lý SVG
│   ├── dom/                 \# Biểu diễn cấu trúc SVG (tương tự DOM)
│   │   └── index.ts
│   ├── parser/              \# Phân tích chuỗi SVG thành cấu trúc DOM nội bộ
│   │   └── index.ts
│   ├── path/                \# Phân tích và xử lý dữ liệu path SVG
│   │   └── index.ts
│   ├── style/               \# Phân tích và áp dụng style SVG
│   │   └── index.ts
│   ├── geometry/            \# Tính toán hình học, tessellation cho rendering
│   │   └── index.ts
│   └── index.ts             \# Export từ các submodule của svg
│
├── editor/                  \# Logic đặc thù của editor, trạng thái UI, công cụ
│   ├── state/               \# Quản lý trạng thái ứng dụng, lịch sử, vùng chọn
│   │   └── index.ts
│   ├── tool/                \# Các công cụ chỉnh sửa (chọn, vẽ đường, hình khối)
│   │   └── index.ts
│   ├── command/             \# Command pattern cho undo/redo
│   │   └── index.ts
│   ├── view/                \# Quản lý viewport (zoom, pan)
│   │   └── index.ts
│   └── index.ts             \# Export từ các submodule của editor
│
├── worker/                  \# Mã nguồn liên quan đến Web Worker
│   ├── manager/             \# Quản lý worker pool, phân phối tác vụ
│   │   └── index.ts
│   ├── task/                \# Định nghĩa các tác vụ offload cho worker
│   │   └── index.ts
│   ├── serialize/           \# (De)serialization cho message worker (tập trung zero-copy)
│   │   └── index.ts
│   └── index.ts             \# Export từ các submodule của worker
│
├── platform/                \# Tích hợp đặc thù của trình duyệt
│   ├── input/               \# Xử lý sự kiện chuột, bàn phím
│   │   └── index.ts
│   ├── file/                \# Truy cập file hệ thống (nếu có)
│   │   └── index.ts
│   └── index.ts
│
├── ui/                      \# Các thành phần giao diện người dùng (nếu không dùng framework ngoài)
│   ├── atom/                \# Các phần tử UI cơ bản (button, input)
│   │   └── index.ts
│   ├── molecule/            \# (Tương tự Atomic Design)
│   │   └── index.ts
│   ├── organism/            \# (Tương tự Atomic Design)
│   │   └── index.ts
│   └── index.ts
│
└── common/                  \# Tiện ích, kiểu dữ liệu, hằng số dùng chung
├── type/                \# Các TypeScript type, interface (chỉ định nghĩa cấu trúc dữ liệu)
│   └── index.ts
├── constant/            \# Hằng số toàn cục
│   └── index.ts
├── math/                \# Thư viện toán học (vector, matrix - nếu tự viết)
│   └── index.ts
├── error/               \# Các kiểu lỗi tùy chỉnh
│   └── index.ts
└── index.ts             \# Export từ các submodule của common

```
**Role of `index.ts`:** Each `index.ts` file within a module directory re-exports the public components (classes, functions, types, constants) that the module intends to provide externally. This allows for cleaner imports, e.g., `import { Device, Queue } from 'core/engine';` instead of pathing to specific files.

## 5. Submodule Dependency Flow

A typical dependency flow might be:
- **`core`**: Low-level, no dependencies on other business logic modules.
- **`common`**: Used by all other modules, minimal dependencies.
- **`svg`**: Depends on `common`, possibly `core` for GPU-related data prep.
- **`worker`**: Depends on `common`, `svg`.
- **`editor`**: High-level, depends on `core`, `svg`, `worker`, `common`, `platform`.
- **`platform`**: Browser APIs, used by `editor` or others needing environment interaction.
- **`ui`**: Depends on `common`, interacts with `editor`.

## 6. Naming Conventions (BEARD Compliant)

(As previously defined - single, meaningful word, `[Word]able` for behavioral interfaces, etc.)

### 6.1. `fields`: `id`, `position`, `fill`
### 6.2. `parameters`: `object`, `distance`
### 6.3. `interfaces` (Behavioral): `Renderable`, `Serializable`
    - Data shape interfaces: `Point`, `Color`
### 6.4. `classes`: `Document`, `Renderer`
### 6.5. `constants`: `Version`, `Pi`, `Defaultradius`
### 6.6. `types` (Aliases & Generics): `Identifier`, `Container<Element>`
### 6.7. `functions`: `initialize`, `parse`

## 7. Key Architectural Considerations

### 7.1. Zero-Copy Data Handling
(As previously defined)

### 7.2. Thread Safety with Web Workers
(As previously defined)

## 8. Suggested Single-Word Identifier Lists (Đơn từ gợi ý)

This section provides lists of single-word identifiers compliant with BEARD naming rules, categorized by main submodules. These can be used for classes, types (PascalCase), functions, variables (lowercase), or constants (PascalCase for single concepts).

### `src/core/` (WGPU & Rendering)
- Nouns: `Core`, `Graphics`, `Engine`, `Renderer`, `Device`, `Adapter`, `Instance`, `Surface`, `Queue`, `Context`, `Pipeline`, `Shader`, `Buffer`, `Texture`, `Sampler`, `Resource`, `Binding`, `Layout`, `Group`, `Stage`, `Pass`, `Frame`, `Viewport`, `Target`, `Format`, `Limit`, `Feature`, `State`, `Descriptor`, `Configuration`, `Control`, `Manager`, `Handler`, `System`, `Swapchain`, `Framebuffer`, `Depthstencil`, `Multisample`, `Query`, `Timestamp`, `Fence`, `Command`, `Encoder`, `Bundle`, `Memory`, `Allocation`, `Pool`, `Access`, `Usage`, `Flag`, `Mode`, `Type`, `Kind`, `Identifier`, `Handle`, `Offset`, `Size`, `Extent`, `Dimension`, `Origin`, `Color`, `Depth`, `Stencil`, `Blend`, `Raster`, `Vertex`, `Fragment`, `Compute`, `Mesh`, `Primitive`, `Topology`, `Index`, `Attribute`, `Uniform`, `Storage`, `Workgroup`, `Barrier`, `Performance`, `Metric`, `Log`, `Trace`, `Debug`, `Profile`, `Capture`
- Verbs: `initialize`, `create`, `destroy`, `begin`, `end`, `submit`, `present`, `draw`, `dispatch`, `bind`, `set`, `get`, `update`, `configure`, `compile`, `link`, `attach`, `map`, `unmap`, `read`, `write`, `copy`, `clear`, `resolve`, `execute`, `wait`, `poll`, `query`, `enable`, `disable`, `reset`, `flush`, `release`, `acquire`, `validate`, `inspect`

### `src/core/engine/`
- Nouns: `Device`, `Adapter`, `Queue`, `Instance`, `Surface`, `Configuration`, `Request`, `Feature`, `Limit`, `Callback`, `Handler`, `Future`, `State`, `Status`, `Error`, `Creation`, `Context`, `Session`
- Verbs: `request`, `select`, `create`, `connect`, `poll`, `configure`, `lose`, `restore`, `check`

### `src/core/buffer/`
- Nouns: `Buffer`, `Allocation`, `View`, `Slice`, `Range`, `Offset`, `Size`, `Capacity`, `Usage`, `Access`, `Mapping`, `Pointer`, `Data`, `Update`, `Copy`, `Transfer`, `Staging`, `Vertexbuffer`, `Indexbuffer`, `Uniformbuffer`, `Storagebuffer`, `Descriptor`, `Memory`, `Pool`
- Verbs: `create`, `destroy`, `map`, `unmap`, `write`, `read`, `copy`, `update`, `clear`, `resize`, `allocate`

### `src/core/shader/`
- Nouns: `Shader`, `Module`, `Source`, `Code`, `Binary`, `SPIRV`, `WGSL`, `Compilation`, `Linking`, `Stage`, `Vertex`, `Fragment`, `Compute`, `EntryPoint`, `Interface`, `Binding`, `Resource`, `Constant`, `Specialization`, `Reflection`, `Metadata`, `Error`, `Log`, `Cache`, `Manager`, `Loader`
- Verbs: `compile`, `load`, `parse`, `validate`, `reflect`, `link`, `create`, `get`, `resolve`

### `src/core/pipeline/`
- Nouns: `Pipeline`, `Renderpipeline`, `Computepipeline`, `Layout`, `Descriptor`, `State`, `Target`, `Blend`, `Depthstencil`, `Multisample`, `Primitive`, `Rasterization`, `Shader`, `Module`, `EntryPoint`, `Bufferlayout`, `Attributelayout`, `Bindinglayout`, `Group`, `Cache`, `Creation`
- Verbs: `create`, `configure`, `build`, `compile`, `get`, `cache`, `bind`

### `src/core/texture/`
- Nouns: `Texture`, `View`, `Sampler`, `Format`, `Dimension`, `Kind`, `Size`, `Mipmap`, `Layer`, `Array`, `Cubemap`, `Volume`, `Surface`, `Pixel`, `Texel`, `Data`, `Upload`, `Copy`, `Transfer`, `Usage`, `Access`, `Descriptor`, `Allocation`, `Image`, `Framebufferattachment`
- Verbs: `create`, `destroy`, `write`, `read`, `copy`, `generate`, `sample`, `bind`, `configure`

### `src/core/resource/`
- Nouns: `Resource`, `Handle`, `Identifier`, `Type`, `Kind`, `Lifetime`, `Scope`, `Manager`, `Pool`, `Cache`, `Tracker`, `Dependency`, `State`, `Transition`, `Barrier`, `Bindgroup`, `Bindlayout`, `Descriptor`, `Set`, `Entry`, `Visibility`, `Queryset`, `Bundle`, `Encoder`
- Verbs: `create`, `destroy`, `allocate`, `free`, `track`, `manage`, `bind`, `access`

### `src/svg/` (SVG Processing)
- Nouns: `SVG`, `Document`, `Element`, `Node`, `Attribute`, `Style`, `Shape`, `Path`, `Text`, `Image`, `Group`, `Symbol`, `Definition`, `Use`, `Pattern`, `Gradient`, `Filter`, `Mask`, `Clip`, `Marker`, `Viewbox`, `Transform`, `Coordinate`, `Unit`, `Color`, `Opacity`, `Stroke`, `Fill`, `Font`, `Metadata`, `DOM`, `Parser`, `Serializer`, `Renderer`, `Interactor`, `Validator`, `Optimizer`, `Converter`, `Geometry`, `Bounds`, `Matrix`, `Vector`, `Point`, `Size`, `Rectangle`, `Circle`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Curve`, `Arc`, `Segment`, `Command`, `Property`, `Value`, `Selector`, `Rule`, `Stylesheet`, `Cascade`, `Inheritance`, `Layout`, `Animation`, `Event`, `Script`, `Structure`, `Tree`, `Model`, `Representation`, `Context`, `State`, `Error`, `Warning`, `Performance`
- Verbs: `parse`, `serialize`, `render`, `create`, `add`, `remove`, `get`, `set`, `update`, `transform`, `query`, `select`, `calculate`, `measure`, `intersect`, `contain`, `optimize`, `validate`, `convert`, `animate`, `handle`, `traverse`, `build`, `apply`, `resolve`, `draw`, `paint`

### `src/svg/dom/`
- Nouns: `Element`, `Node`, `Attribute`, `Textnode`, `Parent`, `Child`, `Sibling`, `Root`, `Tree`, `Structure`, `Collection`, `List`, `Query`, `Selector`, `Path`, `Shape`, `Group`, `Text`, `Image`, `Symbol`, `Definition`, `Metadata`, `Style`, `Script`, `Namespace`, `Tagname`, `Identifier`, `Reference`, `Hierarchy`
- Verbs: `create`, `append`, `insert`, `remove`, `replace`, `get`, `set`, `query`, `find`, `traverse`, `clone`

### `src/svg/parser/`
- Nouns: `Parser`, `Tokenizer`, `Lexer`, `Stream`, `Buffer`, `Char`, `Token`, `Rule`, `Grammar`, `State`, `Context`, `Handler`, `Builder`, `Stack`, `Error`, `Recovery`, `Position`, `Source`, `Input`, `Mode`, `Entity`, `Comment`, `Doctype`, `CDATA`
- Verbs: `parse`, `tokenize`, `lex`, `consume`, `peek`, `next`, `push`, `pop`, `build`, `handle`, `error`

### `src/svg/path/`
- Nouns: `Path`, `Segment`, `Command`, `Move`, `Line`, `Curve`, `Arc`, `Close`, `Point`, `Coordinate`, `Vector`, `Parameter`, `Data`, `Parser`, `Serializer`, `Builder`, `Iterator`, `Transformer`, `Simplifier`, `Normalizer`, `Length`, `Area`, `Centroid`, `Pointatlength`, `Tangent`, `Normal`, `Boundingbox`
- Verbs: `parse`, `serialize`, `add`, `get`, `transform`, `simplify`, `normalize`, `calculate`, `offset`, `interpolate`

### `src/svg/style/`
- Nouns: `Style`, `Property`, `Value`, `Unit`, `Selector`, `Rule`, `Stylesheet`, `Cascade`, `Inheritance`, `Specificity`, `Computedvalue`, `Specifiedvalue`, `Initialvalue`, `Attribute`, `Presentation`, `Inline`, `External`, `Parser`, `Resolver`, `Matcher`, `Applier`, `Context`, `Element`, `Declaration`, `Block`, `Importance`, `Color`, `Font`
- Verbs: `parse`, `apply`, `resolve`, `compute`, `match`, `get`, `set`, `inherit`

### `src/svg/geometry/`
- Nouns: `Geometry`, `Shape`, `Point`, `Vector`, `Matrix`, `Transform`, `Rectangle`, `Circle`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Curve`, `Arc`, `Bounds`, `Intersection`, `Union`, `Difference`, `Area`, `Perimeter`, `Centroid`, `Distance`, `Angle`, `Tessellation`, `Triangulation`, `Vertex`, `Index`, `Normal`, `Mesh`, `Algorithm`, `Precision`, `Tolerance`
- Verbs: `calculate`, `transform`, `intersect`, `contain`, `measure`, `tessellate`, `triangulate`, `simplify`, `offset`

### `src/editor/` (Editor Logic)
- Nouns: `Editor`, `Document`, `View`, `Canvas`, `Tool`, `Selection`, `History`, `Command`, `Action`, `State`, `Mode`, `Context`, `Session`, `User`, `Input`, `Event`, `Handler`, `Manager`, `Controller`, `Model`, `Project`, `File`, `Asset`, `Layer`, `Object`, `Property`, `Inspector`, `Palette`, `Panel`, `Workspace`, `Setting`, `Preference`, `Configuration`, `Plugin`, `Extension`, `Undo`, `Redo`, `Clipboard`, `Copy`, `Paste`, `Cut`, `Delete`, `Drag`, `Drop`, `Resize`, `Rotate`, `Scale`, `Skew`, `Align`, `Distribute`, `Group`, `Ungroup`, `Zoom`, `Pan`, `Grid`, `Snap`, `Ruler`, `Guide`, `Export`, `Import`, `Save`, `Load`, `Render`, `Preview`, `Feedback`, `Notification`, `Progress`, `Error`
- Verbs: `open`, `close`, `save`, `load`, `create`, `delete`, `select`, `deselect`, `edit`, `update`, `apply`, `execute`, `undo`, `redo`, `copy`, `paste`, `cut`, `draw`, `move`, `resize`, `rotate`, `zoom`, `pan`, `snap`, `align`, `group`, `render`, `export`, `import`, `handle`, `dispatch`, `notify`, `configure`, `register`, `activate`

### `src/editor/state/`
- Nouns: `State`, `Store`, `Document`, `Selection`, `History`, `Stack`, `Mutation`, `Action`, `Reducer`, `Effect`, `Subscription`, `Snapshot`, `Patch`, `Diff`, `Current`, `Previous`, `Transaction`, `Scope`, `Context`, `Model`, `Data`, `Tree`, `Change`, `Event`, `Log`, `Persistence`, `Hydration`, `Atom`, `Selector`
- Verbs: `dispatch`, `commit`, `select`, `subscribe`, `get`, `set`, `update`, `reset`, `undo`, `redo`, `push`, `apply`

### `src/editor/tool/`
- Nouns: `Tool`, `Selector`, `Pointer`, `Pen`, `Pencil`, `Brush`, `Eraser`, `Line`, `Rectangle`, `Ellipse`, `Polygon`, `Text`, `Path`, `Nodeeditor`, `Eyedropper`, `Zoom`, `Pan`, `Hand`, `Measure`, `Transform`, `Rotatetool`, `Scaletool`, `Mode`, `State`, `Handler`, `Interactor`, `Feedback`, `Cursor`, `Icon`, `Property`, `Option`, `Manager`, `Registry`, `Active`
- Verbs: `activate`, `deactivate`, `selecttool`, `createtool`, `drawtool`, `edittool`, `handleevent`, `click`, `drag`, `move`, `release`, `hover`, `configuretool`

### `src/editor/command/`
- Nouns: `Command`, `Action`, `Operation`, `Transaction`, `Instruction`, `Step`, `Macro`, `History`, `Stack`, `Executor`, `Invoker`, `Receiver`, `Parameter`, `State`, `Memento`, `Undoable`, `Redoable`, `Composite`, `Sequence`, `Factory`, `Registry`, `Type`, `Name`, `Label`, `Result`, `Effect`, `Change`
- Verbs: `execute`, `undo`, `redo`, `apply`, `revert`, `createcmd`, `recordcmd`, `commitcmd`, `serializecmd`

### `src/editor/view/`
- Nouns: `View`, `Viewport`, `Canvas`, `Renderer`, `Camera`, `Projection`, `Transform`, `Matrix`, `Zoomlevel`, `Panoffset`, `Scroll`, `Offset`, `Scale`, `Center`, `Bounds`, `Frame`, `Grid`, `Ruler`, `Guide`, `Overlay`, `Layer`, `Background`, `Cursor`, `Crosshair`, `Feedback`, `Interaction`, `Handler`, `State`, `Update`, `Display`, `Refresh`, `Culling`
- Verbs: `render`, `updateview`, `zoomto`, `panto`, `scrollto`, `setview`, `getview`, `transformcoords`, `projectcoords`, `fitcontent`, `showgrid`, `hidegrid`

### `src/worker/` (Web Workers)
- Nouns: `Worker`, `Thread`, `Task`, `Job`, `Process`, `Pool`, `Manager`, `Dispatcher`, `Scheduler`, `Queue`, `Message`, `Payload`, `Data`, `Buffer`, `Sharedbuffer`, `Arraybuffer`, `Atom`, `Lock`, `Channel`, `Port`, `Communication`, `Protocol`, `Serialization`, `Deserialization`, `Script`, `Module`, `Context`, `Scope`, `Lifecycle`, `State`, `Error`, `Result`, `Promise`, `Progress`, `Cancellation`, `Timeout`, `Priority`, `Strategy`, `Configuration`, `Interface`, `Offload`, `Compute`, `Parallel`, `Concurrent`
- Verbs: `create`, `spawn`, `terminate`, `post`, `send`, `receive`, `handlemsg`, `execute`, `run`, `dispatch`, `schedule`, `cancel`, `await`, `notify`, `synchronize`, `serialize`, `deserialize`, `manage`, `monitor`

### `src/worker/manager/`
- Nouns: `Manager`, `Pool`, `Worker`, `Instance`, `Controller`, `Strategy`, `Allocator`, `Balancer`, `Router`, `Queue`, `Task`, `Job`, `Lifecycle`, `State`, `Availability`, `Capacity`, `Load`, `Metric`, `Health`, `Configuration`, `Policy`, `Supervisor`
- Verbs: `createpool`, `destroypool`, `addworker`, `removeworker`, `acquireworker`, `releaseworker`, `dispatchtask`, `assigntask`, `managepool`

### `src/worker/task/`
- Nouns: `Task`, `Job`, `Unit`, `Work`, `Operation`, `Definition`, `Type`, `Kind`, `Identifier`, `Input`, `Output`, `Parameter`, `Data`, `Payload`, `Result`, `Error`, `State`, `Status`, `Progress`, `Priority`, `Cancellationtoken`, `Context`, `Script`, `Function`, `Callable`, `Runnable`, `Descriptor`
- Verbs: `definetask`, `createtask`, `executetask`, `runtask`, `processtask`, `canceltask`, `completetask`, `failtask`

### `src/worker/serialize/`
- Nouns: `Serializer`, `Deserializer`, `Format`, `Protocol`, `Schema`, `Message`, `Payload`, `Data`, `Buffer`, `Stream`, `Encoder`, `Decoder`, `Strategy`, `Type`, `Registry`, `Version`, `Compatibility`, `Zerocopy`, `Transferable`
- Verbs: `serialize`, `deserialize`, `encode`, `decode`, `pack`, `unpack`, `convert`, `transformdata`

### `src/common/` (Shared Utilities)
- Nouns: `Utility`, `Helper`, `Service`, `Constant`, `Type`, `Interface`, `Enum`, `Struct`, `Function`, `Math`, `Vector`, `Matrix`, `Point`, `Size`, `Rectangle`, `Color`, `String`, `Number`, `Boolean`, `Date`, `Time`, `Duration`, `Interval`, `Regex`, `Pattern`, `Error`, `Exception`, `Result`, `Option`, `Validation`, `Parser`, `Formatter`, `Converter`, `Generator`, `Iterator`, `Collection`, `List`, `Map`, `Set`, `Queue`, `Stack`, `Tree`, `Graph`, `Node`, `Event`, `Signal`, `Dispatcher`, `Listener`, `Observer`, `Promise`, `Logger`, `Tracer`, `Config`, `Setting`, `Environment`, `Version`, `Identifier`, `UUID`, `Hash`, `Cache`, `Debounce`, `Throttle`, `Retry`, `Timeout`, `Locale`, `Platform`, `URL`, `Path`, `File`, `Stream`, `Encoding`, `Crypto`, `Random`, `Permission`, `State`, `Context`, `Decorator`
- Verbs: `create`, `get`, `set`, `is`, `as`, `to`, `from`, `parse`, `format`, `validate`, `convert`, `generate`, `calculate`, `compare`, `equals`, `sort`, `search`, `filter`, `map`, `reduce`, `each`, `log`, `trace`, `debug`, `warn`, `error`, `assert`, `check`, `resolve`, `reject`, `await`, `sleep`, `debounce`, `throttle`, `retry`, `load`, `save`, `read`, `write`, `copy`, `exists`, `Workspace`, `normalize`, `sanitize`, `escape`, `encode`, `decode`, `encrypt`, `decrypt`, `hash`, `random`, `clamp`, `lerp`

```

**`memories.csv`:**

```csv
timestamp,category,key,value
2025-05-18T14:03:36Z,INITIAL_REQUEST,UserPrompt,"Thiết kế cây phân cấp thư mục fields, parameters, [abstracts]able, [interfaces]able, classes, constants, type associated generics, functions cửa dự án editor svg typescript phải có hiệu xuất cao sử dụng WGPU và zero-copy và an toàn luồng worker browser hạn chế sử dụng thư viện bên ngoài"
2025-05-18T14:03:36Z,PROJECT_CONSTRAINT,Performance,"High performance using WGPU"
2025-05-18T14:03:36Z,PROJECT_CONSTRAINT,DataHandling,"Zero-copy architecture"
2025-05-18T14:03:36Z,PROJECT_CONSTRAINT,Concurrency,"Thread-safe with browser workers"
2025-05-18T14:03:36Z,PROJECT_CONSTRAINT,Dependencies,"Minimal external libraries"
2025-05-18T14:03:36Z,PROJECT_CONSTRAINT,Language,"TypeScript"
2025-05-18T14:03:36Z,NAMING_RULE_INTERPRETATION,[abstracts]able & [interfaces]able,"Corresponds to BEARD Rule 2: [Word]able for traits/interfaces defining behavior/capability."
2025-05-18T14:03:36Z,NAMING_RULE_INTERPRETATION,type associated generics,"Interpret as type aliases or generic types in TypeScript, adhering to single-word rule if newly defined by AI."
2025-05-18T14:03:36Z,NAMING_RULE_INTERPRETATION,fields, parameters, classes, constants, functions,"Standard programming concepts, names must adhere to single-word rule."
2025-05-18T14:03:36Z,NAMING_DETAIL,fields,"Class/object properties, single meaningful word, lowercase."
2025-05-18T14:03:36Z,NAMING_DETAIL,parameters,"Function/method arguments, single meaningful word, lowercase."
2025-05-18T14:03:36Z,NAMING_DETAIL,interfaces,"Behavioral interfaces: [Word]able or [Word]ible. Data shape interfaces: single PascalCase word."
2025-05-18T14:03:36Z,NAMING_DETAIL,classes,"Single meaningful noun, PascalCase (for the single word)."
2025-05-18T14:03:36Z,NAMING_DETAIL,constants,"Single word. PascalCase for single conceptual units (e.g. Version). Lowercase single word if compound concept cannot be single PascalCase word (e.g. defaultbackground)."
2025-05-18T14:03:36Z,NAMING_DETAIL,types,"Type aliases: single meaningful PascalCase word. Generic parameters: conventional (T, K, V) or single PascalCase word if more descriptive."
2025-05-18T14:03:36Z,NAMING_DETAIL,functions,"Single meaningful verb or action-representing word, lowercase."
2025-05-18T14:03:36Z,DESIGN_PRINCIPLE,ZeroCopy,"Emphasize ArrayBuffer/SharedArrayBuffer, avoid serialization overhead for workers and WGPU."
2025-05-18T14:03:36Z,DESIGN_PRINCIPLE,ThreadSafety,"Web Workers with SharedArrayBuffer/Atomics or ArrayBuffer transfer. Synchronize WGPU access."
2025-05-18T21:13:51Z,USER_REQUEST_EXTENSION,Enhancements,"Add index.ts, submodule dependencies, extensive single-word name lists."
2025-05-18T21:13:51Z,DESIGN_DECISION,IndexFiles,"Incorporate index.ts for module exports and organization into architecture.md."
2025-05-18T21:13:51Z,DESIGN_CLARIFICATION,Dependencies,"Explain typical submodule dependency flow in architecture.md."
2025-05-18T21:13:51Z,CONTENT_GENERATION,WordLists,"Generated single-word identifier lists for key submodules in architecture.md, adhering to BEARD rules."
```

**`todo.csv`:**

```csv
id,parent_id,task,status,priority,created_at,updated_at,notes
1,,Define overall project structure (BEARD Philosophy),DONE,1,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
2,1,Design top-level directory structure,DONE,1,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,Updated with index.ts
3,1,Design 'core' module for rendering and WGPU interop,PARTIAL,1,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,Word lists provided. Full interface/type design pending.
4,1,Design 'worker' module for concurrent processing,PARTIAL,1,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,Word lists provided. Full interface/type design pending.
5,1,Design 'state' module for application state management,PARTIAL,1,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,Word lists provided. Full interface/type design pending.
6,1,Design 'svg' module for SVG parsing and model representation,PARTIAL,1,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,Word lists provided. Full interface/type design pending.
7,1,Define naming conventions application for TypeScript based on user's request & BEARD rules,DONE,1,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
8,7,Detail `fields` naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
9,7,Detail `parameters` naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
10,7,Detail `[abstracts]able` / `[interfaces]able` (traits/interfaces) naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,BEARD Rule 2 applied.
11,7,Detail `classes` naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
12,7,Detail `constants` naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,Nuanced application of single-word PascalCase vs lowercase.
13,7,Detail `type associated generics` (type aliases/generics) naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
14,7,Detail `functions` naming and examples,DONE,2,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
15,1,Explain zero-copy considerations with WGPU and ArrayBuffers/SharedArrayBuffers,DONE,1,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
16,1,Explain thread safety with Web Workers and SharedArrayBuffers/Atomics,DONE,1,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
17,,Generate initial response structure,DONE,1,2025-05-18T14:03:36Z,2025-05-18T14:03:36Z,
18,,Detail specific interfaces and types for `core/engine`,TODO,2,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,e.g. Device, Queue, Swapchain abstractions. Word list helps.
19,,Detail specific interfaces and types for `core/buffer`,TODO,2,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,e.g. BufferDescription, BufferUsage. Word list helps.
20,,Detail specific SVG element representations in `svg/dom`,TODO,2,2025-05-18T14:03:36Z,2025-05-18T21:13:51Z,e.g. Path, Rect, Circle. Word list helps.
21,,Incorporate `index.ts` files into directory structure,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
22,,Clarify submodule dependencies,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added section to architecture.md.
23,,Generate single-word identifier lists for core module,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
24,,Generate single-word identifier lists for svg module,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
25,,Generate single-word identifier lists for editor module,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
26,,Generate single-word identifier lists for worker module,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
27,,Generate single-word identifier lists for common module,DONE,1,2025-05-18T21:13:51Z,2025-05-18T21:13:51Z,Added to architecture.md.
28,,Design detailed interfaces for `core/engine` (e.g., `Deviceable`, `Queueable`),TODO,2,2025-05-18T21:13:51Z,,Using provided word lists.
29,,Design data structures for `svg/dom` elements,TODO,2,2025-05-18T21:13:51Z,,Using provided word lists.
30,,Define specific task interfaces in `worker/task`,TODO,2,2025-05-18T21:13:51Z,,Using provided word lists.
```