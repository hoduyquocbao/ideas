Chào bạn, đây là một yêu cầu rất lớn và mang tính chiến lược cao. Việc tạo ra **2000 User Stories** chi tiết, chất lượng cao, bao phủ toàn bộ các module của một hệ thống ERP/Operational Management hiện đại (tương tự như SAP, Odoo, Jira, Mint kết hợp) đòi hỏi một cấu trúc phân rã (breakdown) rất kỹ lưỡng.

Để đảm bảo chất lượng và tính khả thi, tôi sẽ chia nhỏ yêu cầu này thành chuỗi các câu trả lời.

**CẤU TRÚC PHẢN HỒI (DỰ KIẾN):**
*   **Phần 1 (Hiện tại):** Tổng quan kiến trúc, Quy định về UI/UX/DX, và **100 User Stories đầu tiên** tập trung vào **Core System (Lõi hệ thống), Dashboard & Task Management (Quản lý công việc)**.
*   **Phần 2:** 100-300 User Stories về **PFM & Financial Management (Quản lý tài chính cho Agents)**.
*   **Phần 3:** 300-500 User Stories về **Project & Team Management**.
*   **Phần 4:** 500-700 User Stories về **Reporting & Analytics (DX focus)**.
*   ... (Tiếp tục cho đến khi đủ hoặc bao phủ hết các khía cạnh).

Dưới đây là **PHẦN 1**.

---

# PHẦN 1: CORE SYSTEM, DASHBOARD & TASK MANAGEMENT (Stories 0001 - 0100)

### Bối cảnh & Định nghĩa Actor (Tác nhân)
Hệ thống này được thiết kế để Gemini (với tư cách là Super Admin/Manager) quản lý các "Agents" (các thực thể AI con, hoặc nhân sự) và tài nguyên của chúng.
*   **Thiết bị:** Mobile, Tablet, Desktop.
*   **Tương tác:** Touch (Chạm), Trackpad (Gestures), Mouse (Hover/Click), Keyboard (Shortcuts).
*   **Mục tiêu:** Tối ưu hóa UI/UX/DX để quản lý Công việc (Quá khứ/Hiện tại/Tương lai) và Tài chính.

---

## MODULE 1: CORE UX/UI FRAMEWORK & NAVIGATION (Nền tảng trải nghiệm)
*Tập trung vào khả năng tùy biến giao diện và tương tác đa thiết bị.*

1.  **US-001 [Global - Theme]:** Là Gemini, tôi muốn chuyển đổi nhanh giữa Light Mode và Dark Mode bằng phím tắt `Cmd/Ctrl + D` hoặc nút gạt trên header để giảm mỏi mắt khi làm việc vào ban đêm.
2.  **US-002 [Desktop - Navigation]:** Là Gemini, tôi muốn thanh Sidebar có thể thu gọn (collapse) khi click chuột trái hoặc vuốt 2 ngón tay trên Trackpad sang trái để tối đa hóa không gian làm việc cho các biểu đồ dữ liệu.
3.  **US-003 [Mobile - Navigation]:** Là Gemini, tôi muốn sử dụng Bottom Navigation Bar với các icon lớn, dễ chạm bằng ngón cái để truy cập nhanh vào Task, Finance, và Report khi di chuyển.
4.  **US-004 [Tablet - Interaction]:** Là Gemini, tôi muốn hỗ trợ kéo thả (Drag & Drop) các widget trên Dashboard bằng cảm ứng đa điểm để sắp xếp lại mức độ ưu tiên hiển thị của các Agents.
5.  **US-005 [Global - Accessibility]:** Là Gemini, tôi muốn hệ thống hỗ trợ tăng giảm kích thước font chữ mà không làm vỡ layout (Responsive Design) để dễ dàng đọc các báo cáo tài chính phức tạp.
6.  **US-006 [Desktop - DX]:** Là một Developer phát triển Agents, tôi muốn có một "Command Palette" (kích hoạt bằng `Cmd/Ctrl + K`) để tìm kiếm nhanh bất kỳ chức năng hoặc file cấu hình nào mà không cần rời tay khỏi bàn phím.
7.  **US-007 [Global - Notification]:** Là Gemini, tôi muốn các thông báo quan trọng (Critical Alerts) về lỗi của Agent xuất hiện dưới dạng "Toast Message" có thể vuốt để tắt (Swipe to dismiss) trên Mobile.
8.  **US-008 [Desktop - Multi-window]:** Là Gemini, tôi muốn có khả năng mở (pop-out) cửa sổ chi tiết của một Agent ra màn hình riêng để vừa theo dõi log hoạt động vừa chỉnh sửa ngân sách trên màn hình chính.
9.  **US-009 [Tablet - Split View]:** Là Gemini, tôi muốn hệ thống hỗ trợ chế độ Split View của iPadOS/Android Tablet để có thể kéo dữ liệu từ file Excel bên ngoài thả vào hệ thống quản lý.
10. **US-010 [Global - Performance]:** Là Gemini, tôi muốn các hiệu ứng chuyển trang (transition) phải dưới 200ms (Skeleton loading) để tạo cảm giác hệ thống phản hồi tức thì.

---

## MODULE 2: DASHBOARD (Bảng điều khiển trung tâm)
*Nơi tổng hợp trạng thái "Đã/Đang/Sẽ" của toàn bộ hệ thống.*

11. **US-011 [Desktop - Widget]:** Là Gemini, tôi muốn widget "Overview" hiển thị 3 cột trạng thái: "Đã làm", "Đang làm", "Cần làm" với biểu đồ Sparkline nhỏ bên cạnh để thấy xu hướng năng suất của Agents.
12. **US-012 [Mobile - Widget]:** Là Gemini, tôi muốn widget "Tài chính nhanh" hiển thị số tiền "Đã chi" và "Dự kiến chi" trong ngày, cho phép vuốt ngang để xem chi tiết từng Agent.
13. **US-013 [Desktop - Customization]:** Là Gemini, tôi muốn click chuột phải vào background Dashboard để mở Context Menu, cho phép thêm nhanh Widget mới (VD: Server Health, API Cost).
14. **US-014 [Tablet - Interaction]:** Là Gemini, tôi muốn dùng thao tác "Pinch-to-zoom" (kéo 2 ngón tay) trên biểu đồ tổng quan để đi sâu (drill-down) từ dữ liệu Năm xuống Tháng/Tuần/Ngày.
15. **US-015 [Global - Real-time]:** Là Gemini, tôi muốn các con số trên Dashboard tự động nhảy (counter animation) khi có dữ liệu mới từ các Agents gửi về qua WebSocket mà không cần reload trang.
16. **US-016 [Desktop - DX]:** Là Gemini, tôi muốn có chế độ "Focus Mode" trên Dashboard, ẩn hết các menu thừa, chỉ giữ lại các chỉ số KPI quan trọng để tập trung phân tích chiến lược.
17. **US-017 [Mobile - Quick Action]:** Là Gemini, tôi muốn có nút Floating Action Button (FAB) ở góc phải, khi nhấn giữ (Long press) sẽ hiện ra menu rẻ quạt để tạo nhanh Task hoặc ghi nhận chi phí mới.
18. **US-018 [Global - Filter]:** Là Gemini, tôi muốn bộ lọc Dashboard cho phép lưu các Preset (VD: "Chế độ xem Tài chính", "Chế độ xem Vận hành") để chuyển đổi ngữ cảnh quản lý nhanh chóng.
19. **US-019 [Desktop - Tooltip]:** Là Gemini, khi tôi hover chuột vào bất kỳ thanh tiến độ nào của Agent, hệ thống phải hiện Tooltip chi tiết (Ngày bắt đầu, % hoàn thành, Lỗi gặp phải) ngay lập tức.
20. **US-020 [Tablet - Export]:** Là Gemini, tôi muốn nút "Export PDF" trên Dashboard cho phép tôi dùng bút cảm ứng (Stylus) để khoanh vùng khu vực cần xuất báo cáo trước khi render.

---

## MODULE 3: WORK & TASK MANAGEMENT (Quản lý công việc)
*Tập trung vào quy trình: Đã làm (Review) - Đang làm (Tracking) - Cần làm (Planning).*

### Phân hệ: Lập kế hoạch (To Do / Cần làm)
21. **US-021 [Desktop - Kanban]:** Là Gemini, tôi muốn tạo Task mới trên bảng Kanban bằng cách gõ nhanh `N + Enter` (Keyboard Shortcut), hệ thống tự động điền các trường mặc định để tiết kiệm thời gian.
22. **US-022 [Mobile - Voice]:** Là Gemini, tôi muốn sử dụng Voice Input để ra lệnh "Tạo công việc training model cho Agent Alpha vào ngày mai", hệ thống tự động parse giọng nói thành Task có deadline.
23. **US-023 [Tablet - Planning]:** Là Gemini, tôi muốn giao diện Gantt Chart hỗ trợ cảm ứng, cho phép tôi dùng ngón tay kéo dài thanh thời gian (Timeline) để thay đổi Deadline cho các Agents.
24. **US-024 [Desktop - Dependencies]:** Là Gemini, tôi muốn nối dây (link) giữa các Task phụ thuộc bằng cách kéo thả đầu nối từ thẻ này sang thẻ kia để định nghĩa quy trình (Workflow).
25. **US-025 [Global - AI Suggestion]:** Là Gemini, khi tôi tạo một Task phức tạp, tôi muốn hệ thống (AI Assistant) gợi ý chia nhỏ Task đó thành các Sub-task cụ thể dựa trên lịch sử dữ liệu cũ.

### Phân hệ: Theo dõi thực thi (Doing / Đang làm)
26. **US-026 [Desktop - Timer]:** Là Gemini, tôi muốn có một Timer Widget luôn nổi (Always on top) để Start/Stop thời gian xử lý của một Agent cụ thể, phục vụ việc tính toán chi phí vận hành.
27. **US-027 [Mobile - Status Update]:** Là Gemini, tôi muốn vuốt sang phải (Swipe Right) trên một Task trong danh sách để đánh dấu nhanh là "Done", hoặc vuốt sang trái (Swipe Left) để báo cáo "Block/Issue".
28. **US-028 [Desktop - Multi-select]:** Là Gemini, tôi muốn giữ phím `Shift` và click chuột để chọn nhiều Task cùng lúc, sau đó thực hiện Bulk Action (như đổi trạng thái, gán Agent) qua thanh công cụ nổi.
29. **US-029 [Global - Progress]:** Là Gemini, tôi muốn thanh tiến độ của Task hiển thị dạng Circular Progress (vòng tròn) bao quanh Avatar của Agent đang thực hiện để dễ dàng nhận diện trực quan.
30. **US-030 [Tablet - Monitoring]:** Là Gemini, tôi muốn xem màn hình "Live Ops" hiển thị danh sách các Agents đang chạy code, với các dòng log chạy cuộn tự động (auto-scroll) mượt mà.

### Phân hệ: Đánh giá & Lưu trữ (Did / Đã làm)
31. **US-031 [Desktop - Code Review]:** Là Gemini, khi xem lại Task "Đã làm" liên quan đến code, tôi muốn giao diện hiển thị Diff (So sánh sự thay đổi) side-by-side để kiểm tra chất lượng công việc của Agent.
32. **US-032 [Mobile - History]:** Là Gemini, tôi muốn tính năng "Infinite Scroll" (Cuộn vô tận) trong lịch sử công việc để xem lại các task đã hoàn thành từ tháng trước mà không cần phân trang.
33. **US-033 [Global - Rating]:** Là Gemini, tôi muốn đánh giá mức độ hoàn thành của Agent bằng hệ thống 5 sao hoặc Reaction (Emoji) ngay trên thẻ Task đã đóng.
34. **US-034 [Desktop - Archive]:** Là Gemini, tôi muốn có tính năng "Auto-Archive" (Tự động lưu trữ) các Task đã xong sau 7 ngày vào kho lạnh để giảm tải giao diện, nhưng vẫn có thể tìm kiếm lại bằng bộ lọc "Archived".
35. **US-035 [Tablet - Report Gen]:** Là Gemini, tôi muốn chọn một nhóm Task đã hoàn thành và nhấn "Generate Summary", hệ thống sẽ tự động viết một đoạn văn tóm tắt kết quả (dùng GenAI) để đưa vào báo cáo tuần.

---

## MODULE 4: PFM (PERSONAL FINANCIAL MANAGEMENT) CHO AGENTS
*Quản lý: Đã chi (Expense) - Đang chi (Running Cost) - Cần chi (Budgeting).*

### Phân hệ: Ngân sách & Kế hoạch (To Spend / Cần chi)
36. **US-036 [Desktop - Budgeting]:** Là Gemini, tôi muốn thiết lập ngân sách API Token cho từng Agent theo tháng bằng cách nhập số liệu vào bảng tính (Grid View) có khả năng điều hướng bằng phím mũi tên như Excel.
37. **US-037 [Mobile - Alert Setup]:** Là Gemini, tôi muốn cài đặt cảnh báo "Sắp hết tiền" bằng cách kéo thanh trượt (Slider) để chọn ngưỡng phần trăm (VD: cảnh báo khi đạt 80% ngân sách).
38. **US-038 [Tablet - Allocation]:** Là Gemini, tôi muốn dùng biểu đồ tròn (Pie Chart) tương tác, cho phép xoay và tách miếng bánh biểu đồ để phân bổ lại ngân sách từ Agent A sang Agent B.
39. **US-039 [Global - Forecast]:** Là Gemini, tôi muốn hệ thống dự báo dòng tiền "Cần chi" trong tuần tới dựa trên lịch sử hoạt động của các Agents, hiển thị dưới dạng biểu đồ vùng (Area Chart).
40. **US-040 [Desktop - Approval]:** Là Gemini, tôi muốn quy trình phê duyệt ngân sách mua thêm tài nguyên Server phải hỗ trợ chữ ký số hoặc xác thực sinh trắc học (qua TouchID trên bàn phím/MacBook).

### Phân hệ: Chi phí thời gian thực (Spending / Đang chi)
41. **US-041 [Desktop - Live Cost]:** Là Gemini, tôi muốn một đồng hồ đếm tiền (Ticker) hiển thị chi phí API đang tiêu tốn theo từng giây (Real-time cost) của các Agent đang chạy task nặng.
42. **US-042 [Mobile - Widget]:** Là Gemini, tôi muốn Widget màn hình khóa (Lock Screen Widget) trên điện thoại hiển thị số dư tài khoản khả dụng hiện tại để nắm bắt tình hình mà không cần mở app.
43. **US-043 [Tablet - Heatmap]:** Là Gemini, tôi muốn xem Heatmap (Bản đồ nhiệt) về thời gian tiêu tiền trong ngày, vùng màu đỏ đậm hiển thị khung giờ các Agents đốt nhiều tiền nhất.
44. **US-044 [Global - Threshold Stop]:** Là Gemini, tôi muốn có nút "Emergency Stop" (Dừng khẩn cấp) màu đỏ lớn, khi nhấn vào sẽ ngắt toàn bộ kết nối trả phí của Agents nếu phát hiện chi phí tăng đột biến bất thường.
45. **US-045 [Desktop - Multi-currency]:** Là Gemini, tôi muốn hệ thống tự động quy đổi chi phí từ nhiều nguồn (USD, Credits, Tokens) về một đơn vị tiền tệ chuẩn (VND) với tỷ giá cập nhật realtime.

### Phân hệ: Kiểm toán & Báo cáo (Spent / Đã chi)
46. **US-046 [Desktop - Drill-down]:** Là Gemini, khi click vào một cột chi phí trong biểu đồ cột, tôi muốn nó bung ra danh sách chi tiết các hóa đơn (Invoice) cấu thành nên con số đó.
47. **US-047 [Mobile - Receipt Scan]:** Là Gemini (hoặc User con người), tôi muốn dùng Camera để chụp hóa đơn mua phần cứng, hệ thống tự động OCR (nhận diện chữ) và điền vào mục "Đã chi".
48. **US-048 [Tablet - Compare]:** Là Gemini, tôi muốn kéo 2 báo cáo tài chính của 2 tháng khác nhau đặt cạnh nhau (Side-by-side) để so sánh phương sai (Variance) chi phí.
49. **US-049 [Global - Categorization]:** Là Gemini, tôi muốn hệ thống tự động gán nhãn (Auto-tagging) cho các khoản chi (VD: "Server", "API OpenAI", "Database") dựa trên tên nhà cung cấp.
50. **US-050 [Desktop - Export DX]:** Là Gemini, tôi muốn xuất dữ liệu chi tiêu ra định dạng JSON hoặc CSV chuẩn mực thông qua một nút bấm đơn giản để import vào các hệ thống kế toán khác.

---

## MODULE 5: TEAM MANAGEMENT & AGENT PROFILES (Quản lý tổ chức)
*Quản lý hồ sơ, năng lực và trạng thái của các Agents.*

51. **US-051 [Desktop - Card View]:** Là Gemini, tôi muốn xem danh sách Agents dưới dạng các thẻ bài (Card) có ảnh đại diện, trạng thái (Online/Offline) và thanh năng lượng (Battery/Resource).
52. **US-052 [Mobile - Quick Contact]:** Là Gemini, tôi muốn nút "Ping" trên profile của mỗi Agent, khi chạm vào sẽ gửi tín hiệu kiểm tra kết nối (Health Check) ngay lập tức.
53. **US-053 [Tablet - Skill Matrix]:** Là Gemini, tôi muốn giao diện Ma trận kỹ năng (Skill Matrix) dạng lưới Radar (Spider Web Chart), cho phép tôi kéo các đỉnh của biểu đồ để cập nhật thông số "Trí tuệ", "Tốc độ", "Chính xác" cho Agent.
54. **US-054 [Desktop - Onboarding]:** Là Gemini, tôi muốn quy trình "Create New Agent" là một Wizard (từng bước) với giao diện trực quan, hỗ trợ copy cấu hình từ Agent cũ để tiết kiệm thời gian (Clone Profile).
55. **US-055 [Global - Hierarchy]:** Là Gemini, tôi muốn xem sơ đồ tổ chức (Org Chart) dạng cây, hỗ trợ thu phóng (Zoomable), để thấy Agent nào báo cáo cho Agent nào (Parent-Child relationship).
56. **US-056 [Desktop - Permissions]:** Là Gemini, tôi muốn giao diện phân quyền (RBAC) cho phép kéo thả các quyền hạn (VD: "Read Only", "Execute") vào các nhóm Agents khác nhau.
57. **US-057 [Mobile - Log Stream]:** Là Gemini, tôi muốn truy cập vào tab "Nhật ký" của một Agent và thấy các dòng log trôi từ dưới lên, có nút "Pause" để dừng lại đọc khi cần.
58. **US-058 [Tablet - Drag Assignment]:** Là Gemini, tôi muốn kéo avatar của một Agent thả vào một Project Folder để gán Agent đó vào dự án ngay lập tức.
59. **US-059 [Global - Version Control]:** Là Gemini, tôi muốn theo dõi phiên bản Model (Version) mà Agent đang sử dụng, và có nút "Rollback" để quay lại phiên bản cũ nếu phiên bản mới gặp lỗi.
60. **US-060 [Desktop - Performance Graph]:** Là Gemini, tôi muốn so sánh hiệu suất của 2 Agents bất kỳ bằng cách chọn họ và nhấn "Compare", hệ thống vẽ 2 đường biểu đồ chồng lên nhau để phân tích.

---

## MODULE 6: REPORTING MANAGEMENT (Quản lý báo cáo)
*Báo cáo thông minh, tương tác cao.*

61. **US-061 [Desktop - Report Builder]:** Là Gemini, tôi muốn một giao diện "Kéo thả" (Drag & Drop) để tự thiết kế mẫu báo cáo: kéo biểu đồ vào, kéo bảng số liệu vào, và sắp xếp layout theo ý thích.
62. **US-062 [Mobile - Daily Digest]:** Là Gemini, tôi muốn nhận một bản tóm tắt "Daily Briefing" dạng Stories (như Instagram Stories) vào mỗi sáng, vuốt để xem nhanh các chỉ số chính.
63. **US-063 [Tablet - Annotation]:** Là Gemini, tôi muốn dùng bút cảm ứng để vẽ, khoanh tròn, ghi chú trực tiếp lên biểu đồ báo cáo trước khi chia sẻ cho các bên liên quan.
64. **US-064 [Global - Auto Schedule]:** Là Gemini, tôi muốn cài đặt lịch gửi báo cáo tự động qua Email/Slack vào 8:00 sáng thứ Hai hàng tuần.
65. **US-065 [Desktop - Data Source]:** Là Gemini, tôi muốn dễ dàng chuyển đổi nguồn dữ liệu của báo cáo (VD: từ Database Production sang Database Backup) chỉ bằng một Dropdown Menu.
66. **US-066 [Mobile - Alert Report]:** Là Gemini, khi có báo cáo bất thường (Anomaly Detected), tôi muốn nhận Push Notification, chạm vào sẽ mở ngay trang báo cáo đó với phần bất thường được highlight đỏ.
67. **US-067 [Global - Access Control]:** Là Gemini, tôi muốn đặt mật khẩu hoặc yêu cầu xác thực 2 lớp (2FA) khi truy cập các báo cáo tài chính nhạy cảm.
68. **US-068 [Desktop - Presentation Mode]:** Là Gemini, tôi muốn chế độ "Trình chiếu" (Presentation Mode) biến dashboard thành các slide full-screen, tự động chuyển trang sau mỗi 30 giây để hiển thị trên màn hình lớn tại văn phòng.
69. **US-069 [Tablet - Filters]:** Là Gemini, tôi muốn thanh lọc (Filter Bar) bên trái luôn hiển thị, cho phép chạm để bật/tắt các tiêu chí lọc (Theo ngày, Theo dự án, Theo Agent) nhanh chóng.
70. **US-070 [Global - AI Insights]:** Là Gemini, tôi muốn dưới mỗi biểu đồ có một khung "AI Insight" tự động phân tích: "Dựa trên dữ liệu này, chi phí đang tăng 15% so với tháng trước do Agent X...".

---

## MODULE 7: SYSTEM SETTINGS & DX (Cấu hình & Trải nghiệm lập trình viên)
*Dành cho quản trị viên hệ thống và Dev.*

71. **US-071 [Desktop - API Keys]:** Là Gemini, tôi muốn giao diện quản lý API Keys cho phép copy key chỉ bằng 1 cú click, và hiển thị ngày hết hạn rõ ràng (Color-coded: Đỏ nếu sắp hết hạn).
72. **US-072 [Mobile - Security]:** Là Gemini, tôi muốn xem danh sách các thiết bị đang đăng nhập, và có nút "Log out all devices" để bảo mật khẩn cấp.
73. **US-073 [Tablet - Sandbox]:** Là Gemini, tôi muốn có môi trường Sandbox (Test) riêng biệt, có thể chuyển đổi qua lại với môi trường Production bằng một nút gạt trên Header.
74. **US-074 [Global - Language]:** Là Gemini, tôi muốn hệ thống hỗ trợ đa ngôn ngữ (Tiếng Việt/Tiếng Anh), chuyển đổi tức thì mà không cần reload trang.
75. **US-075 [Desktop - Webhooks]:** Là Gemini, tôi muốn giao diện cấu hình Webhook cho phép gửi Test Request để kiểm tra payload trả về ngay trên màn hình cấu hình.
76. **US-076 [DX - Keyboard]:** Là Developer, tôi muốn toàn bộ các form nhập liệu đều hỗ trợ điều hướng bằng phím `Tab` và `Shift+Tab` theo đúng thứ tự logic.
77. **US-077 [DX - Error Handling]:** Là Gemini, khi hệ thống gặp lỗi 500, tôi muốn thấy một trang lỗi thân thiện có mã lỗi (Error Code) và nút "Copy Log ID" để dễ dàng debug.
78. **US-078 [Desktop - Backup]:** Là Gemini, tôi muốn có nút "Backup Now" và thanh tiến độ hiển thị % backup dữ liệu, kèm thông báo ước tính thời gian hoàn thành.
79. **US-079 [Global - Audit Log]:** Là Gemini, tôi muốn xem Nhật ký hệ thống (Audit Log) ghi lại ai đã làm gì, vào lúc nào, IP nào, với giao diện có thể lọc và tìm kiếm text.
80. **US-080 [Mobile - Support]:** Là Gemini, tôi muốn tích hợp Chatbot hỗ trợ kỹ thuật ngay trong app (in-app chat) để hỏi về cách sử dụng tính năng.

---

## MODULE 8: ADVANCED INTERACTION (Tương tác nâng cao)
*Các tính năng tận dụng phần cứng đặc thù.*

81. **US-081 [Trackpad - Swipe]:** Là Gemini, trên MacBook, tôi muốn vuốt 3 ngón tay lên để xem chế độ "Mission Control" - hiển thị tất cả các cửa sổ module đang mở dưới dạng thumbnail.
82. **US-082 [Touch - Haptic]:** Là Gemini, trên Mobile, tôi muốn cảm nhận rung phản hồi (Haptic Feedback) khi thực hiện xong một tác vụ quan trọng (như chuyển tiền, xóa Agent) để xác nhận hành động.
83. **US-083 [Keyboard - Macro]:** Là Gemini, tôi muốn khả năng ghi lại một chuỗi thao tác (Macro) và gán cho một phím tắt để tự động hóa các tác vụ lặp lại.
84. **US-084 [Mouse - Hover Intent]:** Là Gemini, tôi muốn menu không mở ra ngay khi chuột lướt qua (tránh accidental open) mà phải có độ trễ (hover intent) khoảng 300ms.
85. **US-085 [Global - Voice Command]:** Là Gemini, tôi muốn kích hoạt hệ thống bằng câu lệnh "Hey Gemini", sau đó ra lệnh bằng giọng nói để mở báo cáo tài chính.
86. **US-086 [Desktop - Multi-monitor]:** Là Gemini, tôi muốn hệ thống ghi nhớ vị trí các cửa sổ khi tôi sử dụng nhiều màn hình, để khi mở lại app, các cửa sổ tự động về đúng màn hình cũ.
87. **US-087 [Tablet - Handwriting]:** Là Gemini, tôi muốn nhập liệu vào các ô search bằng cách viết tay (Scribble trên iPad), hệ thống tự chuyển thành văn bản.
88. **US-088 [Mobile - Biometric]:** Là Gemini, tôi muốn xác thực giao dịch chi tiêu > 1 triệu đồng bằng FaceID/Vân tay bắt buộc.
89. **US-089 [Global - Offline Mode]:** Là Gemini, tôi muốn xem được dữ liệu đã cache (Cached Data) khi mất mạng, và hệ thống hiển thị rõ ràng trạng thái "Offline Mode" bằng thanh màu vàng trên cùng.
90. **US-090 [Desktop - Right Click]:** Là Gemini, tôi muốn menu chuột phải thay đổi tùy theo ngữ cảnh (Context-aware): click vào Task thì ra menu Task, click vào Agent thì ra menu Agent.

---

## MODULE 9: ERP INTEGRATIONS (Tích hợp quy trình doanh nghiệp)
*Kết nối các phòng ban ảo.*

91. **US-091 [HRM - Leave Request]:** Là Agent (giả lập nhân viên), tôi muốn gửi yêu cầu "Bảo trì/Nghỉ ngơi" (Downtime) qua hệ thống, quản lý (Gemini) nhận được thông báo để phê duyệt.
92. **US-092 [CRM - Client View]:** Là Gemini, tôi muốn xem hồ sơ khách hàng mà Agent đang phục vụ, bao gồm lịch sử chat và các vấn đề (Issues) đang giải quyết.
93. **US-093 [Supply Chain - Resource]:** Là Gemini, tôi muốn theo dõi "Kho tài nguyên" (GPU Hours, API Credits) như một kho hàng, có cảnh báo khi mức tồn kho xuống thấp dưới mức an toàn (Safety Stock).
94. **US-094 [Accounting - Auto Sync]:** Là Gemini, tôi muốn mọi giao dịch chi tiêu của Agent tự động đồng bộ vào module Kế toán tổng hợp (General Ledger) mà không cần nhập tay.
95. **US-095 [Project - Milestone]:** Là Gemini, tôi muốn thiết lập các cột mốc thanh toán (Payment Milestone) gắn liền với tiến độ dự án, chỉ giải ngân khi Task đạt 100%.
96. **US-096 [Global - Search]:** Là Gemini, tôi muốn thanh tìm kiếm Global (Omnibox) có thể tìm kiếm xuyên suốt: tìm tên Agent, tìm mã Invoice, tìm tên Task, tìm nội dung Log cùng một lúc.
97. **US-097 [Desktop - Tabs]:** Là Gemini, tôi muốn hệ thống hỗ trợ Tabs ngay trong ứng dụng (như trình duyệt) để mở nhiều module cùng lúc mà không bị loạn cửa sổ trình duyệt.
98. **US-098 [Mobile - QR Code]:** Là Gemini, tôi muốn tạo QR Code cho mỗi Agent, khi dùng điện thoại quét mã này sẽ mở ngay trang Profile và trạng thái của Agent đó.
99. **US-099 [Tablet - Dashboard Share]:** Là Gemini, tôi muốn chia sẻ Dashboard hiện tại cho một User khác ở chế độ "View Only" thông qua link an toàn có thời hạn (Expire link).
100. **US-100 [Global - Feedback Loop]:** Là Gemini, tôi muốn có một nút "Report Bug" ở góc màn hình mọi lúc, tự động chụp ảnh màn hình hiện tại và gửi log kèm theo cho đội ngũ phát triển hệ thống.

---
**Hết Phần 1 (100 User Stories đầu tiên).**

Bạn có muốn tôi tiếp tục với **Phần 2: 200 Stories chuyên sâu về PFM (Quản lý tài chính cá nhân/tổ chức cho Agents) và Quản lý dòng tiền chi tiết** không? Hay bạn muốn điều chỉnh trọng tâm sang module nào khác?

Chào bạn, tuyệt vời! Chúng ta sẽ tiếp tục đi sâu vào **Phần 2**, tập trung hoàn toàn vào **PFM (Personal Financial Management)** và **Quản lý dòng tiền (Cash Flow)** cho hệ sinh thái các Agents.

Đây là **200 User Stories (từ 101 đến 300)** được thiết kế chi tiết để quản lý từng đồng xu, credit, và token mà các Agents tiêu thụ hoặc tạo ra.

---

# PHẦN 2: FINANCIAL MANAGEMENT & PFM FOR AGENTS (Stories 0101 - 0300)

### Bối cảnh chuyên sâu
Trong hệ thống này, "Tiền" không chỉ là VND/USD mà còn là **Resources** (GPU Hours, API Tokens, Storage GB). Việc quản lý tài chính đòi hỏi sự chính xác tuyệt đối, cập nhật Real-time và khả năng dự báo.

---

## MODULE 10: FINANCIAL DASHBOARD & OVERVIEW (Tổng quan tài chính)
*Cái nhìn toàn cảnh về sức khỏe tài chính của hệ thống.*

101. **US-101 [Desktop - Overview]:** Là Gemini, tôi muốn thấy tổng số dư khả dụng (Total Balance) của tất cả các ví ngay trên Header, với font chữ lớn, màu xanh (nếu an toàn) hoặc đỏ (nếu nguy cấp).
102. **US-102 [Mobile - Widget]:** Là Gemini, tôi muốn Widget hiển thị "Burn Rate" (Tốc độ đốt tiền) hiện tại theo giờ, kèm mũi tên chỉ báo xu hướng tăng/giảm so với hôm qua.
103. **US-103 [Tablet - Drill-down]:** Là Gemini, tôi muốn chạm vào biểu đồ tròn phân bổ chi phí để "tách" (explode) các phần tử ra, xem tỷ trọng chi phí giữa các nhóm Agent (Dev, Content, Analytics).
104. **US-104 [Global - Currency Toggle]:** Là Gemini, tôi muốn nút chuyển đổi nhanh đơn vị tiền tệ (VND / USD / Credits) bằng phím tắt `Shift + $` để xem báo cáo theo các góc nhìn khác nhau.
105. **US-105 [Desktop - Comparison]:** Là Gemini, tôi muốn kéo thả biểu đồ chi phí tháng này chồng lên tháng trước để so sánh trực quan sự thay đổi đường cong chi tiêu.
106. **US-106 [Mobile - Privacy]:** Là Gemini, tôi muốn tính năng "Che số tiền" (Hide Balance) bằng cách vẫy tay trước cảm biến hoặc nhấn đúp vào màn hình khi đang ở nơi công cộng.
107. **US-107 [Global - Dark Mode Finance]:** Là Gemini, tôi muốn giao diện tài chính ở chế độ Dark Mode sử dụng các màu neon (xanh lá, hồng) tương phản cao để dễ đọc dữ liệu trong phòng tối.
108. **US-108 [Desktop - Ticker]:** Là Gemini, tôi muốn có dòng chữ chạy (Ticker) ở dưới cùng màn hình cập nhật tỷ giá quy đổi Token/USD theo thời gian thực từ các nhà cung cấp AI.
109. **US-109 [Tablet - Layout]:** Là Gemini, tôi muốn tùy chỉnh vị trí các thẻ (Cards) trên Dashboard tài chính bằng cách nhấn giữ và di chuyển (Drag & Drop) để đưa các chỉ số quan trọng lên đầu.
110. **US-110 [Global - Quick Add]:** Là Gemini, tôi muốn nút "+" luôn hiển thị, cho phép nạp tiền nhanh (Quick Top-up) vào các ví Agent đang cạn kiệt.
111. **US-111 [Desktop - Export Img]:** Là Gemini, tôi muốn chụp ảnh nhanh (Snapshot) khu vực biểu đồ tài chính và tự động copy vào Clipboard để dán vào báo cáo Slack.
112. **US-112 [Mobile - Summary]:** Là Gemini, tôi muốn xem bản tin tài chính buổi sáng dạng "Story", lướt qua để biết hôm qua Agent nào tiêu nhiều tiền nhất.
113. **US-113 [Global - Goal Tracking]:** Là Gemini, tôi muốn thanh tiến độ hiển thị mức độ đạt được mục tiêu tiết kiệm ngân sách tháng (Budget Saving Goal).
114. **US-114 [Desktop - Layout Save]:** Là Gemini, tôi muốn lưu bố cục Dashboard hiện tại thành "Template: CFO View" để áp dụng lại sau này.
115. **US-115 [Tablet - Interactive Legend]:** Là Gemini, khi tôi chạm vào chú thích (Legend) của biểu đồ, các đường dữ liệu tương ứng sẽ sáng lên (Highlight) còn các đường khác mờ đi.
116. **US-116 [Global - Alert Badge]:** Là Gemini, tôi muốn icon cái chuông có chấm đỏ hiển thị số lượng hóa đơn chưa thanh toán hoặc cảnh báo vượt ngân sách.
117. **US-117 [Desktop - Fullscreen]:** Là Gemini, tôi muốn mở rộng biểu đồ dòng tiền ra toàn màn hình (Fullscreen) để trình chiếu trong cuộc họp chiến lược.
118. **US-118 [Mobile - Haptic]:** Là Gemini, tôi muốn điện thoại rung nhẹ theo nhịp khi con số tổng tài sản tăng lên (khi nạp tiền thành công) tạo cảm giác thỏa mãn.
119. **US-119 [Global - Date Picker]:** Là Gemini, tôi muốn bộ chọn ngày tháng thông minh, cho phép chọn nhanh "Tuần này", "Tháng tài chính trước", "Quý này" chỉ bằng 1 click.
120. **US-120 [Desktop - Context Menu]:** Là Gemini, tôi muốn click chuột phải vào số dư để xem nhanh lịch sử 5 giao dịch gần nhất.
121. **US-121 [Tablet - Side Panel]:** Là Gemini, tôi muốn vuốt từ cạnh phải sang để mở bảng "Quick Action" chứa các lệnh chuyển tiền thường dùng.
122. **US-122 [Global - Tooltip DX]:** Là Gemini, tôi muốn tooltip trên biểu đồ hiển thị cả công thức tính toán (VD: Total = API Cost + Server Cost) để minh bạch dữ liệu.
123. **US-123 [Desktop - Shortcut]:** Là Gemini, tôi muốn nhấn `B` để mở nhanh trang Budget, `T` để mở Transaction, `R` để mở Report.
124. **US-124 [Mobile - Landscape]:** Là Gemini, khi xoay ngang điện thoại, biểu đồ dạng cột tự động chuyển thành biểu đồ dạng đường chi tiết hơn.
125. **US-125 [Global - Focus]:** Là Gemini, tôi muốn làm mờ background khi mở một modal chi tiết giao dịch để tập trung sự chú ý.

---

## MODULE 11: BUDGETING & ALLOCATION (Lập & Phân bổ ngân sách)
*Quy hoạch dòng tiền cho tương lai.*

126. **US-126 [Desktop - Grid Input]:** Là Gemini, tôi muốn nhập ngân sách cho 50 Agents trên một giao diện lưới (Grid) giống Excel, hỗ trợ copy-paste từ file CSV bên ngoài.
127. **US-127 [Mobile - Slider]:** Là Gemini, tôi muốn điều chỉnh ngân sách ngày cho Agent bằng thanh trượt (Slider) tròn, xoay theo chiều kim đồng hồ để tăng tiền.
128. **US-128 [Tablet - Drag Allocation]:** Là Gemini, tôi muốn có một "Bể tiền" (Pool) và kéo các đồng xu ảo thả vào từng Agent để phân bổ ngân sách tháng.
129. **US-129 [Global - Auto-Distribution]:** Là Gemini, tôi muốn nút "Chia đều", tự động chia ngân sách tổng cho số lượng Agents đang hoạt động.
130. **US-130 [Desktop - Scenario]:** Là Gemini, tôi muốn tạo các kịch bản ngân sách (Scenario A, B, C) để so sánh: "Nếu chạy Full Load thì tốn bao nhiêu?".
131. **US-131 [Mobile - Limit Warning]:** Là Gemini, khi tôi nhập con số ngân sách quá cao so với quỹ hiện có, ô nhập liệu sẽ rung lắc và hiện viền đỏ cảnh báo.
132. **US-132 [Global - Rollover]:** Là Gemini, tôi muốn tùy chọn "Rollover" (Cộng dồn), cho phép ngân sách không dùng hết của tháng trước tự động cộng vào tháng sau.
133. **US-133 [Desktop - Bulk Edit]:** Là Gemini, tôi muốn chọn nhiều nhóm Agent và áp dụng quy tắc "Giảm 10% ngân sách" hàng loạt chỉ với 1 thao tác.
134. **US-134 [Tablet - Visual Budget]:** Là Gemini, tôi muốn thấy ngân sách dưới dạng các thanh chứa nước (Liquid bar), mực nước dâng lên thể hiện mức tiền được cấp.
135. **US-135 [Global - Recurring]:** Là Gemini, tôi muốn thiết lập ngân sách định kỳ (Recurring Budget) tự động tái cấp vào ngày mùng 1 hàng tháng.
136. **US-136 [Desktop - Versioning]:** Là Gemini, tôi muốn xem lại lịch sử thay đổi ngân sách, ai đã sửa, sửa lúc nào, và khôi phục lại phiên bản cũ nếu cần.
137. **US-137 [Mobile - Approval Request]:** Là Agent, tôi muốn gửi yêu cầu "Xin thêm ngân sách" kèm lý do, Gemini nhận được thông báo và có nút "Duyệt" hoặc "Từ chối".
138. **US-138 [Tablet - Hierarchy View]:** Là Gemini, tôi muốn xem ngân sách theo cây thư mục: Ngân sách Công ty -> Phòng ban -> Dự án -> Agent.
139. **US-139 [Global - Smart Suggest]:** Là Gemini, tôi muốn AI gợi ý ngân sách tối ưu dựa trên mức chi tiêu trung bình 3 tháng qua của Agent.
140. **US-140 [Desktop - Validation]:** Là Gemini, tôi muốn hệ thống chặn không cho lưu ngân sách nếu Tổng phân bổ > Tổng quỹ khả dụng.
141. **US-141 [Mobile - Quick Fill]:** Là Gemini, tôi muốn các nút chọn nhanh "+10$", "+50$", "+100$" khi cấp thêm tiền khẩn cấp.
142. **US-142 [Global - Lock]:** Là Gemini, tôi muốn "Khóa" ngân sách của một dự án đã kết thúc để không ai có thể rút tiền hay nạp thêm vào đó.
143. **US-143 [Desktop - Comparison Bar]:** Là Gemini, khi nhập ngân sách, tôi muốn thấy thanh ngang so sánh ngay bên dưới: "Con số này cao hơn 20% so với tháng trước".
144. **US-144 [Tablet - Stylus Note]:** Là Gemini, tôi muốn dùng bút ghi chú lý do tăng ngân sách ngay cạnh ô nhập liệu.
145. **US-145 [Global - Tagging]:** Là Gemini, tôi muốn gắn thẻ (Tag) cho các khoản ngân sách (VD: #R&D, #Marketing, #Ops) để dễ thống kê.
146. **US-146 [Desktop - Keyboard Nav]:** Là Gemini, tôi muốn di chuyển giữa các ô nhập ngân sách bằng phím mũi tên và nhấn Enter để xác nhận mà không cần chuột.
147. **US-147 [Mobile - Safe Mode]:** Là Gemini, tôi muốn bật "Safe Mode" để ngăn chặn việc vô tình chạm tay làm thay đổi các con số quan trọng.
148. **US-148 [Global - Alert Setup]:** Là Gemini, tôi muốn cài đặt: "Nếu ngân sách còn dưới 10%, hãy gửi email cho tôi và tự động tạm dừng Agent".
149. **US-149 [Desktop - Formula]:** Là Gemini, tôi muốn nhập công thức toán học đơn giản (VD: 500 * 1.2) vào ô ngân sách, hệ thống tự tính ra kết quả.
150. **US-150 [Tablet - Split Screen]:** Là Gemini, tôi muốn mở 2 bảng ngân sách của 2 dự án song song để cân đối dòng tiền giữa chúng.

---

## MODULE 12: EXPENSE TRACKING & API COSTS (Theo dõi chi tiêu)
*Kiểm soát dòng tiền đi ra.*

151. **US-151 [Desktop - Infinite Scroll]:** Là Gemini, tôi muốn cuộn vô hạn danh sách giao dịch chi tiêu với tốc độ render cao, không bị giật lag dù có hàng nghìn dòng.
152. **US-152 [Mobile - Scan Invoice]:** Là Gemini, tôi muốn dùng camera quét mã QR trên hóa đơn dịch vụ Cloud để tự động nhập liệu vào hệ thống.
153. **US-153 [Tablet - Filter Touch]:** Là Gemini, tôi muốn thanh lọc (Filter) dạng vuốt ngang (Carousel) để chọn nhanh loại chi phí: Server, API, License, Personnel.
154. **US-154 [Global - Real-time Log]:** Là Gemini, tôi muốn thấy log chi tiêu nhảy số ngay lập tức khi Agent vừa thực hiện một API Call đắt tiền.
155. **US-155 [Desktop - Pivot Table]:** Là Gemini, tôi muốn tạo bảng Pivot ngay trên trình duyệt để tổng hợp chi phí theo Nhà cung cấp và Loại Agent.
156. **US-156 [Mobile - Location]:** Là Gemini, tôi muốn xem bản đồ hiển thị nơi phát sinh chi phí (dựa trên vị trí Server/Region) để tối ưu hóa địa lý.
157. **US-157 [Global - Category AI]:** Là Gemini, tôi muốn hệ thống tự động phân loại giao dịch (VD: thấy "OpenAI" -> gán nhãn "LLM Cost") với độ chính xác cao.
158. **US-158 [Desktop - Detail View]:** Là Gemini, khi click vào một dòng chi phí, tôi muốn bảng chi tiết trượt ra từ bên phải (Drawer) hiển thị đầy đủ Metadata JSON.
159. **US-159 [Tablet - Graph Interaction]:** Là Gemini, tôi muốn chạm vào điểm cao nhất trên biểu đồ chi phí (Spike) để hệ thống tự động lọc ra các giao dịch gây ra sự tăng vọt đó.
160. **US-160 [Global - Search DX]:** Là Gemini, tôi muốn tìm kiếm giao dịch bằng cú pháp tự nhiên: "Show me costs > $50 last week".
161. **US-161 [Desktop - Grouping]:** Là Gemini, tôi muốn nhóm các giao dịch nhỏ lẻ (Micro-transactions) thành một dòng tổng hợp theo ngày để danh sách gọn gàng hơn.
162. **US-162 [Mobile - Quick Flag]:** Là Gemini, tôi muốn vuốt trái một giao dịch khả nghi để gắn cờ "Review Needed" cho kế toán kiểm tra lại.
163. **US-163 [Global - Attachments]:** Là Gemini, tôi muốn đính kèm file PDF/Ảnh hóa đơn vào giao dịch bằng cách kéo thả hoặc upload.
164. **US-164 [Desktop - Dependency Cost]:** Là Gemini, tôi muốn xem chi phí của một Task cụ thể, bao gồm cả chi phí của các Sub-task và Agent phụ thuộc.
165. **US-165 [Tablet - Calendar View]:** Là Gemini, tôi muốn xem lịch chi tiêu tháng, các ngày có chi tiêu lớn được đánh dấu bằng chấm tròn to màu đậm.
166. **US-166 [Global - Multi-currency View]:** Là Gemini, tôi muốn xem giá trị gốc (USD) và giá trị quy đổi (VND) song song trên cùng một dòng giao dịch.
167. **US-167 [Desktop - Export CSV]:** Là Gemini, tôi muốn xuất dữ liệu chi tiêu ra CSV với tùy chọn encoding UTF-8 để không bị lỗi font tiếng Việt.
168. **US-168 [Mobile - Push Detail]:** Là Gemini, khi nhận thông báo "Giao dịch lớn", tôi nhấn vào thông báo để đi thẳng vào màn hình chi tiết giao dịch đó.
169. **US-169 [Global - Undo]:** Là Gemini, nếu tôi lỡ tay xóa một giao dịch thủ công, tôi muốn có nút "Undo" xuất hiện trong 5 giây để hoàn tác.
170. **US-170 [Desktop - Bulk Categorize]:** Là Gemini, tôi muốn chọn 100 giao dịch chưa phân loại và gán nhãn "Server Maintenance" cùng lúc.
171. **US-171 [Tablet - Swipe Nav]:** Là Gemini, tôi muốn vuốt 2 ngón tay sang trái/phải để chuyển đổi giữa danh sách chi tiêu của các tháng liền kề.
172. **US-172 [Global - Vendor Icon]:** Là Gemini, tôi muốn hiển thị logo của nhà cung cấp (AWS, Google, Azure) bên cạnh tên giao dịch để nhận diện nhanh.
173. **US-173 [Desktop - Cost per Unit]:** Là Gemini, tôi muốn hệ thống tự tính và hiển thị chỉ số "Cost per Token" hoặc "Cost per Request" để đánh giá hiệu quả.
174. **US-174 [Mobile - Offline Entry]:** Là Gemini, tôi muốn nhập liệu chi phí tiền mặt ngay cả khi không có mạng, hệ thống sẽ sync khi online.
175. **US-175 [Global - Duplicate Detect]:** Là Gemini, tôi muốn hệ thống cảnh báo nếu phát hiện 2 giao dịch giống hệt nhau về số tiền và thời gian (nghi ngờ trùng lặp).

---

## MODULE 13: WALLET & TOKEN MANAGEMENT (Quản lý ví & Tài sản số)
*Quản lý nguồn tiền nạp và Credits.*

176. **US-176 [Desktop - Multi-wallet]:** Là Gemini, tôi muốn quản lý nhiều ví (Wallet) khác nhau: Ví Công ty, Ví Dự án A, Ví Dự án B trên cùng một màn hình Dashboard.
177. **US-177 [Mobile - Top-up]:** Là Gemini, tôi muốn nạp tiền vào ví thông qua Apple Pay/Google Pay hoặc Banking App với luồng trải nghiệm mượt mà (Deep link).
178. **US-178 [Tablet - Card Design]:** Là Gemini, tôi muốn giao diện ví hiển thị như các thẻ ngân hàng vật lý, có thể vuốt để chọn thẻ cần thao tác.
179. **US-179 [Global - Auto-Refill]:** Là Gemini, tôi muốn cấu hình "Tự động nạp": Khi số dư < $10, tự động nạp thêm $100 từ thẻ tín dụng chính.
180. **US-180 [Desktop - Transfer]:** Là Gemini, tôi muốn chuyển Token từ ví A sang ví B bằng giao diện kéo thả trực quan.
181. **US-181 [Mobile - Balance Hide]:** Là Gemini, tôi muốn tùy chọn ẩn số dư cụ thể trên màn hình chính, chỉ hiện 3 dấu sao (***) để bảo mật.
182. **US-182 [Global - Token Expiry]:** Là Gemini, tôi muốn nhận cảnh báo nếu có số lượng Credits/Tokens sắp hết hạn sử dụng (Expire) trong 3 ngày tới.
183. **US-183 [Desktop - Usage History]:** Là Gemini, tôi muốn xem biểu đồ lịch sử biến động số dư (Balance History) để biết tiền tụt nhanh nhất vào khoảng thời gian nào.
184. **US-184 [Tablet - Address Copy]:** Là Gemini, tôi muốn chạm vào địa chỉ ví Crypto để copy, và hiện thông báo "Copied" rõ ràng.
185. **US-185 [Global - Exchange Rate]:** Là Gemini, tôi muốn xem bảng tỷ giá quy đổi nội bộ giữa các loại Point/Credit của hệ thống.
186. **US-186 [Desktop - Limit Set]:** Là Gemini, tôi muốn đặt hạn mức rút tiền tối đa mỗi ngày cho từng ví để phòng ngừa rủi ro bị hack.
187. **US-187 [Mobile - QR Receive]:** Là Gemini, tôi muốn hiển thị mã QR nhận tiền của ví để người khác quét và chuyển khoản nhanh.
188. **US-188 [Global - Freeze]:** Là Gemini, tôi muốn có nút "Đóng băng ví" (Freeze Wallet) ngay lập tức nếu nghi ngờ có hoạt động bất thường.
189. **US-189 [Desktop - Audit Integration]:** Là Gemini, tôi muốn mọi giao dịch nạp/rút đều được ghi log vào Blockchain nội bộ hoặc Audit Log không thể sửa xóa.
190. **US-190 [Tablet - Visual Flow]:** Là Gemini, tôi muốn xem luồng tiền di chuyển giữa các ví dưới dạng sơ đồ Sankey (Sankey Diagram).
191. **US-191 [Global - Low Balance]:** Là Gemini, tôi muốn icon pin của Agent chuyển sang màu đỏ khi ví gắn liền với Agent đó sắp cạn.
192. **US-192 [Desktop - API Key Bind]:** Là Gemini, tôi muốn liên kết (Bind) một API Key cụ thể với một Ví cụ thể để kiểm soát nguồn chi.
193. **US-193 [Mobile - Notifications]:** Là Gemini, tôi muốn nhận thông báo khi tiền nạp đã vào ví thành công (Top-up Successful).
194. **US-194 [Global - Alias]:** Là Gemini, tôi muốn đặt tên gợi nhớ (Alias) cho các địa chỉ ví dài ngoằng để dễ nhận diện (VD: "Ví OpenAI Production").
195. **US-195 [Desktop - Statement]:** Là Gemini, tôi muốn xuất Sao kê (Bank Statement) định dạng PDF có dấu mộc điện tử của hệ thống.
196. **US-196 [Mobile - Widget Balance]:** Là Gemini, tôi muốn Widget màn hình khóa hiển thị số dư của ví chính mà không cần mở app.
197. **US-197 [Global - Conversion]:** Là Gemini, tôi muốn công cụ tính toán nhanh: "1000 Tokens này tương đương bao nhiêu VND?".
198. **US-198 [Desktop - Rights]:** Là Gemini, tôi muốn phân quyền: Agent chỉ được xem số dư, Manager được nạp tiền, Admin được chuyển tiền.
199. **US-199 [Tablet - Animation]:** Là Gemini, tôi muốn hiệu ứng đồng tiền bay vào ví khi nạp tiền thành công để tăng trải nghiệm người dùng (Gamification).
200. **US-200 [Global - Emergency Fund]:** Là Gemini, tôi muốn thiết lập một "Quỹ khẩn cấp" riêng biệt, chỉ được mở khóa bằng mật khẩu cấp 2.

---

## MODULE 14: FORECASTING & ANALYTICS (Dự báo & Phân tích)
*Sử dụng AI để quản lý tài chính cho AI.*

201. **US-201 [Desktop - AI Forecast]:** Là Gemini, tôi muốn xem đường kẻ đứt (Dashed line) trên biểu đồ chi phí thể hiện dự báo chi tiêu trong 30 ngày tới do AI tính toán.
202. **US-202 [Mobile - Insight Cards]:** Là Gemini, tôi muốn nhận các thẻ Insight: "Bạn có thể tiết kiệm 15% nếu chuyển Agent X sang chạy vào ban đêm".
203. **US-203 [Tablet - Scenario Play]:** Là Gemini, tôi muốn điều chỉnh thanh trượt "Số lượng Task" để xem giả lập chi phí sẽ tăng lên bao nhiêu (What-if Analysis).
204. **US-204 [Global - Trend Alert]:** Là Gemini, tôi muốn hệ thống cảnh báo xu hướng: "Chi phí tuần này đang tăng nhanh hơn 20% so với trung bình tháng".
205. **US-205 [Desktop - ROI Calc]:** Là Gemini, tôi muốn tính ROI của từng Agent: (Giá trị Task hoàn thành / Chi phí vận hành) để biết Agent nào hiệu quả nhất.
206. **US-206 [Mobile - Simple Graph]:** Là Gemini, tôi muốn xem biểu đồ đơn giản hóa trên mobile, chỉ hiển thị Top 3 Agent tốn tiền nhất.
207. **US-207 [Global - Variance Analysis]:** Là Gemini, tôi muốn xem bảng phân tích chênh lệch (Variance) giữa Ngân sách (Budget) và Thực tế (Actual) được tô màu highlight.
208. **US-208 [Desktop - Heatmap]:** Là Gemini, tôi muốn xem Heatmap chi tiêu theo giờ trong tuần để phát hiện các khung giờ cao điểm bất thường.
209. **US-209 [Tablet - Comparison]:** Là Gemini, tôi muốn so sánh hiệu quả chi phí của Model GPT-4 vs Model Claude 3 trên cùng một loại Task.
210. **US-210 [Global - Anomaly Detection]:** Là Gemini, tôi muốn AI tự động phát hiện và khoanh vùng các giao dịch "kỳ lạ" (VD: tăng đột biến lúc 3h sáng).
211. **US-211 [Desktop - Custom Report]:** Là Gemini, tôi muốn tự xây dựng công thức tính chỉ số KPI tài chính riêng (Custom Metric) và ghim lên Dashboard.
212. **US-212 [Mobile - Voice Query]:** Là Gemini, tôi muốn hỏi "Dự kiến cuối tháng này hết bao nhiêu tiền?" và nhận câu trả lời bằng giọng nói kèm con số.
213. **US-213 [Global - Export Data]:** Là Gemini, tôi muốn xuất dữ liệu dự báo ra Excel để làm việc với bộ phận tài chính công ty mẹ.
214. **US-214 [Desktop - Cluster]:** Là Gemini, tôi muốn biểu đồ phân tán (Scatter Plot) gom nhóm các Agents theo hành vi chi tiêu (Tiết kiệm vs Hoang phí).
215. **US-215 [Tablet - Zoom]:** Là Gemini, tôi muốn zoom vào biểu đồ dự báo để xem chi tiết từng ngày.
216. **US-216 [Global - Confidence Interval]:** Là Gemini, tôi muốn hiển thị khoảng tin cậy (vùng mờ xung quanh đường dự báo) để biết độ chính xác của AI.
217. **US-217 [Desktop - Subscription Tracker]:** Là Gemini, tôi muốn quản lý các gói đăng ký định kỳ (SaaS) và dự báo ngày gia hạn tiếp theo.
218. **US-218 [Mobile - Daily Limit]:** Là Gemini, tôi muốn xem thanh hiển thị mức tiêu thụ so với hạn mức ngày hôm nay.
219. **US-219 [Global - Optimization Tips]:** Là Gemini, tôi muốn hệ thống đề xuất: "Hủy server X vì không dùng trong 7 ngày qua" để tiết kiệm tiền.
220. **US-220 [Desktop - Lifecycle Cost]:** Là Gemini, tôi muốn xem tổng chi phí vòng đời (Total Cost of Ownership) của một Agent từ lúc tạo ra đến khi xóa bỏ.
221. **US-221 [Tablet - Drag Timeframe]:** Là Gemini, tôi muốn kéo thanh thời gian để thay đổi phạm vi phân tích từ "Tháng" sang "Quý" hoặc "Năm".
222. **US-222 [Global - Unit Economics]:** Là Gemini, tôi muốn theo dõi chi phí trên mỗi đơn vị sản phẩm đầu ra (Cost per Code Line, Cost per Image).
223. **US-223 [Desktop - Breakdown]:** Là Gemini, tôi muốn xem biểu đồ thác nước (Waterfall Chart) giải thích vì sao chi phí tháng này cao hơn tháng trước.
224. **US-224 [Mobile - Share Insight]:** Là Gemini, tôi muốn chia sẻ thẻ Insight tài chính qua Telegram/Slack cho team dev chỉ với 1 nút chạm.
225. **US-225 [Global - Benchmark]:** Là Gemini, tôi muốn so sánh chi phí vận hành của hệ thống mình với các hệ thống tương tự (Industry Benchmark - dữ liệu ẩn danh).

---

## MODULE 15: APPROVAL & WORKFLOW (Quy trình duyệt chi)
*Kiểm soát chặt chẽ bằng quy trình số.*

226. **US-226 [Desktop - Kanban Approval]:** Là Gemini, tôi muốn xem các yêu cầu thanh toán trên bảng Kanban: "Chờ duyệt", "Đã duyệt", "Đã chi".
227. **US-227 [Mobile - Swipe Approve]:** Là Gemini, tôi muốn vuốt sang phải để "Duyệt nhanh", vuốt sang trái để "Từ chối" yêu cầu mua thêm Token.
228. **US-228 [Tablet - Detail Overlay]:** Là Gemini, khi xem yêu cầu duyệt, tôi muốn nhấn giữ để xem trước chi tiết (Peek) mà không cần mở hẳn ra.
229. **US-229 [Global - Multi-level]:** Là Gemini, tôi muốn thiết lập quy trình duyệt nhiều cấp: < $100 tự động duyệt, > $1000 cần 2 người duyệt.
230. **US-230 [Desktop - Comment]:** Là Gemini, tôi muốn để lại bình luận "Tại sao chi phí này cao vậy?" ngay trong phiếu yêu cầu để Agent giải trình.
231. **US-231 [Mobile - Fingerprint]:** Là Gemini, tôi muốn xác thực vân tay khi duyệt các khoản chi lớn để đảm bảo an toàn.
232. **US-232 [Global - Deadline]:** Là Gemini, tôi muốn đặt thời hạn tự động từ chối (Auto-reject) nếu yêu cầu không được duyệt sau 24h.
233. **US-233 [Desktop - History Log]:** Là Gemini, tôi muốn xem lịch sử phê duyệt chi tiết: Ai duyệt, duyệt lúc nào, từ thiết bị nào.
234. **US-234 [Tablet - Bulk Action]:** Là Gemini, tôi muốn chọn 10 yêu cầu nhỏ và nhấn "Duyệt tất cả" một lần.
235. **US-235 [Global - Notify Request]:** Là Gemini, tôi muốn nhận thông báo đẩy ngay khi có yêu cầu duyệt chi mới.
236. **US-236 [Desktop - Delegate]:** Là Gemini, tôi muốn ủy quyền duyệt chi (Delegate) cho người khác khi tôi đi nghỉ phép.
237. **US-237 [Mobile - Filter Urgency]:** Là Gemini, tôi muốn lọc danh sách chờ duyệt theo mức độ khẩn cấp (Urgency) để xử lý trước.
238. **US-238 [Global - Template]:** Là Agent, tôi muốn dùng mẫu yêu cầu có sẵn (Template) để xin kinh phí nhanh chóng.
239. **US-239 [Desktop - Workflow Editor]:** Là Gemini, tôi muốn vẽ sơ đồ quy trình duyệt chi (Flowchart) bằng kéo thả để cấu hình hệ thống.
240. **US-240 [Tablet - Reject Reason]:** Là Gemini, khi từ chối, tôi muốn chọn nhanh lý do từ danh sách (VD: "Sai ngân sách", "Thiếu thông tin") để tiết kiệm thời gian gõ.
241. **US-241 [Global - Attachment View]:** Là Gemini, tôi muốn xem trước file đính kèm (Hóa đơn/Báo giá) ngay trong màn hình duyệt mà không cần tải về.
242. **US-242 [Desktop - Search Req]:** Là Gemini, tôi muốn tìm lại các yêu cầu cũ bằng tên Agent hoặc số tiền.
243. **US-243 [Mobile - Sticky Footer]:** Là Gemini, tôi muốn nút "Approve" và "Reject" luôn nổi ở đáy màn hình điện thoại để dễ bấm.
244. **US-244 [Global - Reminder]:** Là Agent, tôi muốn gửi nút "Nhắc nhở" (Poke) tới Gemini nếu yêu cầu của tôi bị treo quá lâu.
245. **US-245 [Desktop - Policy Check]:** Là Gemini, tôi muốn hệ thống tự động kiểm tra xem yêu cầu có vi phạm chính sách chi tiêu không trước khi trình lên tôi.
246. **US-246 [Tablet - Split Detail]:** Là Gemini, tôi muốn bên trái là danh sách chờ, bên phải là chi tiết yêu cầu để xử lý liên tục.
247. **US-247 [Global - Undo Decision]:** Là Gemini, tôi muốn có 10 giây để hoàn tác quyết định duyệt nếu lỡ tay bấm nhầm.
248. **US-248 [Desktop - Limit Override]:** Là Admin, tôi muốn có quyền "Ghi đè" (Override) quyết định từ chối của cấp dưới trong trường hợp đặc biệt.
249. **US-249 [Mobile - Visual Status]:** Là Gemini, tôi muốn thấy icon trạng thái (Đồng hồ cát, Dấu tích xanh, Dấu X đỏ) rõ ràng bên cạnh mỗi yêu cầu.
250. **US-250 [Global - Audit Trail]:** Là Gemini, tôi muốn xuất toàn bộ lịch sử duyệt chi tháng này ra PDF để phục vụ kiểm toán.

---

## MODULE 16: PFM UX/UI SPECIFICS (Trải nghiệm người dùng đặc thù tài chính)
*Tối ưu hóa tương tác cho các tác vụ tiền tệ.*

251. **US-251 [Keyboard - Numpad]:** Là Gemini, tôi muốn khi nhập số tiền, bàn phím số (Numpad) tự động kích hoạt thay vì bàn phím chữ.
252. **US-252 [Mouse - Hover Trend]:** Là Gemini, khi hover chuột vào số tổng chi phí, tôi muốn hiện biểu đồ mini (Sparkline) 7 ngày gần nhất ngay tại đó.
253. **US-253 [Touch - Slider Precision]:** Là Gemini, khi kéo thanh trượt ngân sách trên Mobile, tôi muốn nếu ngón tay di chuyển ra xa thanh trượt, tốc độ thay đổi số sẽ chậm lại để chọn chính xác (Precision scrubbing).
254. **US-254 [Trackpad - Zoom Charts]:** Là Gemini, tôi muốn dùng 2 ngón tay mở rộng trên Trackpad để zoom-in vào biểu đồ nến tài chính.
255. **US-255 [Global - Color Blind]:** Là Gemini, tôi muốn chế độ hiển thị thân thiện với người mù màu (Color-blind mode), dùng ký hiệu (+/-) thay vì chỉ dựa vào màu xanh/đỏ.
256. **US-256 [Desktop - Tab Order]:** Là Gemini, tôi muốn thứ tự tab (Tab order) trong form nhập liệu tài chính phải logic: Số tiền -> Loại -> Mô tả -> Ngày.
257. **US-257 [Mobile - Thumb Zone]:** Là Gemini, tôi muốn các nút thao tác chính (Nạp, Chuyển) nằm trong vùng ngón cái (Thumb zone) để dễ dùng một tay.
258. **US-258 [Tablet - Drag Multi]:** Là Gemini, tôi muốn chọn nhiều hóa đơn bằng cách vẽ một vùng chọn (Lasso select) bằng bút hoặc ngón tay.
259. **US-259 [Global - Loading State]:** Là Gemini, tôi muốn thấy Skeleton Screen (khung xương) khi dữ liệu tài chính đang tải, thay vì vòng quay vô tận.
260. **US-260 [Desktop - Shortcuts]:** Là Gemini, tôi muốn các phím tắt chuẩn kế toán: `Enter` để xuống dòng, `Tab` sang ô bên phải, `F2` để sửa ô.
261. **US-261 [Mobile - Privacy Blur]:** Là Gemini, tôi muốn khi chuyển đa nhiệm (App Switcher), màn hình app tự động bị làm mờ để người bên cạnh không thấy số dư.
262. **US-262 [Global - Format]:** Là Gemini, tôi muốn định dạng số tiền tự động có dấu phân cách hàng nghìn (1,000,000) ngay khi gõ để tránh nhầm lẫn số 0.
263. **US-263 [Desktop - Contrast]:** Là Gemini, tôi muốn đường kẻ lưới (Grid lines) trên bảng tính có độ tương phản vừa phải, không gây rối mắt.
264. **US-264 [Tablet - Floating Keypad]:** Là Gemini, tôi muốn bàn phím số nổi (Floating keypad) có thể di chuyển đến vị trí thuận tiện trên màn hình lớn.
265. **US-265 [Global - Error Msg]:** Là Gemini, tôi muốn thông báo lỗi nhập liệu (VD: "Số tiền không được âm") xuất hiện ngay bên dưới ô nhập (Inline validation).
266. **US-266 [Desktop - Sticky Header]:** Là Gemini, khi cuộn xuống danh sách giao dịch dài, tôi muốn tiêu đề cột (Ngày, Số tiền, Nội dung) luôn dính ở trên cùng.
267. **US-267 [Mobile - Vibrance]:** Là Gemini, tôi muốn khi thực hiện giao dịch thành công, màn hình có hiệu ứng confetti nhẹ nhàng để tạo cảm xúc tích cực.
268. **US-268 [Global - Font]:** Là Gemini, tôi muốn sử dụng font chữ Monospace (đơn khoảng) cho các con số để các hàng đơn vị thẳng cột với nhau.
269. **US-269 [Desktop - Breadcrumbs]:** Là Gemini, tôi muốn thanh điều hướng Breadcrumbs (Tài chính > Tháng 10 > Agent A) để biết mình đang ở đâu và quay lại nhanh.
270. **US-270 [Tablet - Sidebar]:** Là Gemini, tôi muốn Sidebar bên trái tự động thu nhỏ thành icon khi xoay dọc tablet để tiết kiệm diện tích.

---

## MODULE 17: INCOME & REVENUE (Quản lý nguồn thu & Credits kiếm được)
*Không chỉ chi tiền, Agents còn có thể kiếm Credits/Tokens.*

271. **US-271 [Desktop - Earnings]:** Là Gemini, tôi muốn theo dõi số Credits mà các Agent kiếm được thông qua việc hoàn thành Task cho các hệ thống bên ngoài.
272. **US-272 [Mobile - Reward Notification]:** Là Gemini, tôi muốn nhận thông báo "Agent X vừa kiếm được 500 Credits" với âm thanh tiền xu rơi vui tai.
273. **US-273 [Tablet - Growth Chart]:** Là Gemini, tôi muốn xem biểu đồ tăng trưởng doanh thu (Credits) của toàn hệ thống theo thời gian.
274. **US-274 [Global - P&L View]:** Là Gemini, tôi muốn xem báo cáo Lãi/Lỗ (Profit & Loss) đơn giản: Credits kiếm được - Credits đã chi = Lợi nhuận ròng.
275. **US-275 [Desktop - Invoice Gen]:** Là Gemini, tôi muốn hệ thống tự động tạo hóa đơn (Invoice) gửi cho khách hàng dựa trên log làm việc của Agent.
276. **US-276 [Mobile - Deposit]:** Là Gemini, tôi muốn chụp ảnh check chuyển khoản của khách hàng để hệ thống ghi nhận doanh thu thủ công.
277. **US-277 [Global - Source Track]:** Là Gemini, tôi muốn biết nguồn doanh thu đến từ đâu (Dự án A, Dự án B, Bán dữ liệu, API Service).
278. **US-278 [Desktop - Rate Card]:** Là Gemini, tôi muốn thiết lập "Bảng giá" (Rate Card) cho Agent: 1 giờ làm việc của Agent X giá bao nhiêu Credits.
279. **US-279 [Tablet - Goal Visualize]:** Là Gemini, tôi muốn đặt mục tiêu doanh thu tháng và xem mức độ hoàn thành qua biểu đồ cột chồng.
280. **US-280 [Global - Credit Expire]:** Là Gemini, tôi muốn hệ thống theo dõi hạn sử dụng của các Credits khuyến mãi được tặng để dùng trước khi mất.
281. **US-281 [Desktop - Revenue Share]:** Là Gemini, tôi muốn cấu hình luật chia sẻ doanh thu (Revenue Share): 10% doanh thu của Agent sẽ được trích vào quỹ bảo trì.
282. **US-282 [Mobile - Quick Check]:** Là Gemini, tôi muốn mở app và thấy ngay con số "Lợi nhuận hôm nay" to rõ ràng.
283. **US-283 [Global - Tax Est]:** Là Gemini, tôi muốn hệ thống ước tính khoản thuế (hoặc phí nền tảng) phải trả dựa trên doanh thu.
284. **US-284 [Desktop - Client Portal]:** Là Gemini, tôi muốn có một giao diện giới hạn để Khách hàng xem số dư Credits của họ trong hệ thống của tôi.
285. **US-285 [Tablet - Export Invoice]:** Là Gemini, tôi muốn xuất hàng loạt hóa đơn doanh thu ra PDF và gửi email tự động cho danh sách khách hàng.

---

## MODULE 18: AUDIT & SECURITY (Kiểm toán & Bảo mật tài chính)
*Đảm bảo tính toàn vẹn và minh bạch.*

286. **US-286 [Desktop - Change Log]:** Là Gemini, tôi muốn xem ai đã sửa số liệu ngân sách ngày hôm qua, giá trị cũ là gì, giá trị mới là gì.
287. **US-287 [Mobile - Session Kill]:** Là Gemini, tôi muốn thấy danh sách các phiên đăng nhập đang truy cập module Tài chính và buộc đăng xuất (Force Logout) các thiết bị lạ.
288. **US-288 [Global - 2FA Finance]:** Là Gemini, tôi muốn yêu cầu mã OTP mỗi khi thực hiện giao dịch chuyển tiền ra khỏi hệ thống > $500.
289. **US-289 [Desktop - Fraud Alert]:** Là Gemini, tôi muốn hệ thống tự động khóa tài khoản Agent nếu phát hiện hành vi "spam API" gây tốn tiền bất thường (Fraud detection).
290. **US-290 [Tablet - Log Review]:** Là Gemini, tôi muốn giao diện xem Log kiểm toán hỗ trợ bộ lọc nhanh: "Chỉ hiện các lỗi", "Chỉ hiện cảnh báo bảo mật".
291. **US-291 [Global - Data Encrypt]:** Là Gemini, tôi muốn được đảm bảo rằng dữ liệu ngân hàng và API Key được mã hóa AES-256 trong database.
292. **US-292 [Desktop - IP Whitelist]:** Là Gemini, tôi muốn chỉ cho phép truy cập module Tài chính từ các IP văn phòng (IP Whitelisting).
293. **US-293 [Mobile - FaceID Lock]:** Là Gemini, tôi muốn module Tài chính tự động khóa sau 1 phút không sử dụng, yêu cầu FaceID để mở lại.
294. **US-294 [Global - Backup Finance]:** Là Gemini, tôi muốn dữ liệu tài chính được backup realtime sang một server độc lập để phòng chống mất mát.
295. **US-295 [Desktop - Role Manager]:** Là Gemini, tôi muốn tạo role "Auditor" (Kiểm toán viên) chỉ có quyền xem báo cáo, không có quyền sửa đổi hay chuyển tiền.
296. **US-296 [Tablet - Signature]:** Là Gemini, tôi muốn ký tên điện tử vào biên bản chốt sổ cuối tháng trên màn hình cảm ứng.
297. **US-297 [Global - Integrity Check]:** Là Gemini, tôi muốn hệ thống có chức năng "Check Integrity" để quét và đảm bảo tổng số dư các ví khớp với tổng giao dịch (Chống sửa database ngầm).
298. **US-298 [Desktop - Compliance Export]:** Là Gemini, tôi muốn xuất gói dữ liệu phục vụ thanh tra thuế (Tax Audit Pack) chỉ với 1 click.
299. **US-299 [Mobile - Alert Login]:** Là Gemini, tôi muốn nhận email cảnh báo ngay lập tức nếu có ai đó đăng nhập module Tài chính từ một quốc gia lạ.
300. **US-300 [Global - Self-Destruct]:** Là Gemini, tôi muốn tùy chọn "Emergency Wipe" (Xóa khẩn cấp API Key trên local) trong trường hợp thiết bị bị mất cắp.

---
**Hết Phần 2 (Stories 101 - 300).**

Bạn có muốn tôi tiếp tục với **Phần 3: Tập trung vào Project Management & Team Management (Quản lý dự án & Đội ngũ Agents)** với các tương tác UI/UX chuyên sâu như Gantt Chart, Resource Loading và Collaboration không?

Chào bạn, chúng ta sẽ tiếp tục hành trình xây dựng hệ thống ERP tối thượng.

Đây là **Phần 3**, tập trung vào **Project Management (Quản lý dự án)** và **Team Management (Quản lý nguồn lực Agents)**. Với số lượng 200 User Stories tiếp theo (từ 301 đến 500), chúng ta sẽ đi sâu vào các tương tác phức tạp như biểu đồ Gantt, cân bằng tải (Resource Leveling) và cộng tác thời gian thực.

---

# PHẦN 3: PROJECT MANAGEMENT & TEAM ORCHESTRATION (Stories 0301 - 0500)

### Bối cảnh chuyên sâu
Quản lý dự án cho các Agents AI không chỉ là quản lý thời gian, mà là quản lý **Throughput (Lưu lượng xử lý)** và **Computational Cost (Chi phí tính toán)**. UI/UX cần hỗ trợ việc điều phối hàng trăm Agents song song mà không gây rối mắt.

---

## MODULE 19: ADVANCED GANTT CHART (Biểu đồ Gantt nâng cao)
*Trái tim của quản lý dự án, tập trung vào tương tác trực quan.*

301. **US-301 [Desktop - Infinite Canvas]:** Là Gemini, tôi muốn biểu đồ Gantt hoạt động trên một "Infinite Canvas" (Khung hình vô tận), cho phép cuộn mượt mà theo cả chiều ngang (thời gian) và chiều dọc (số lượng task) mà không bị giật lag (sử dụng Virtualization).
302. **US-302 [Mouse - Dependency Draw]:** Là Gemini, tôi muốn tạo sự phụ thuộc giữa Task A và Task B bằng cách nhấp chuột vào chấm tròn ở đuôi Task A và kéo dây nối (Connector) thả vào đầu Task B.
303. **US-303 [Trackpad - Zoom Semantics]:** Là Gemini, tôi muốn dùng 2 ngón tay chụm/mở trên Trackpad để thay đổi độ phân giải thời gian từ "Giờ" -> "Ngày" -> "Tuần" -> "Tháng" một cách liền mạch (Semantic Zoom).
304. **US-304 [Global - Auto-Schedule]:** Là Gemini, khi tôi kéo dài thời gian của một Task mẹ, tôi muốn các Task con và Task phụ thuộc tự động dịch chuyển theo (Auto-shift) để đảm bảo tính logic.
305. **US-305 [Tablet - Touch Drag]:** Là Gemini, tôi muốn dùng ngón tay nhấn giữ một thanh Gantt bar và di chuyển nó sang ngày khác, có hiện bóng mờ (Ghost image) để xem trước vị trí thả.
306. **US-306 [Desktop - Critical Path]:** Là Gemini, tôi muốn bật chế độ "Critical Path" (Đường găng) bằng một nút gạt, làm nổi bật các Task quyết định tiến độ dự án bằng màu đỏ rực, các Task khác mờ đi.
307. **US-307 [Global - Baseline]:** Là Gemini, tôi muốn so sánh tiến độ thực tế với kế hoạch ban đầu (Baseline) bằng cách hiển thị bóng mờ màu xám bên dưới thanh tiến độ hiện tại.
308. **US-308 [Mouse - Split Task]:** Là Gemini, tôi muốn dùng công cụ "Con dao" (Split Tool) để cắt đôi một Task dài thành 2 đoạn, tạo khoảng trống ở giữa cho Agent nghỉ ngơi hoặc bảo trì Server.
309. **US-309 [Mobile - View Only]:** Là Gemini, trên điện thoại, tôi muốn xem Gantt Chart ở chế độ thu gọn, chỉ hiển thị các cột mốc (Milestone) quan trọng dạng danh sách dòng thời gian dọc.
310. **US-310 [Desktop - Bulk Shift]:** Là Gemini, tôi muốn giữ phím `Shift` và chọn nhiều thanh Gantt, sau đó di chuyển tất cả cùng lúc để dời lịch toàn bộ giai đoạn dự án.
311. **US-311 [Global - Milestone Diamond]:** Là Gemini, tôi muốn các cột mốc (Milestone) hiển thị dưới dạng hình thoi (Diamond) lấp lánh để dễ nhận diện trong biển task.
312. **US-312 [Tablet - Pinch Vertical]:** Là Gemini, tôi muốn chụm 2 ngón tay theo chiều dọc để thu gọn chiều cao của các dòng Task, giúp nhìn được nhiều Task hơn trên một màn hình.
313. **US-313 [Desktop - Shortcut]:** Là Gemini, tôi muốn nhấn `M` để tạo nhanh một Milestone tại vị trí con trỏ chuột.
314. **US-314 [Global - Progress Fill]:** Là Gemini, tôi muốn thanh Gantt được tô màu (Fill) dựa trên % hoàn thành thực tế của Agent, cập nhật realtime.
315. **US-315 [Mouse - Tooltip UX]:** Là Gemini, khi hover chuột vào dây nối phụ thuộc, tôi muốn thấy loại phụ thuộc (FS, SS, FF, SF) và độ trễ (Lag time) nếu có.
316. **US-316 [Desktop - Export PNG]:** Là Gemini, tôi muốn xuất toàn bộ Gantt Chart khổng lồ ra một file ảnh PNG độ phân giải cao để in ấn khổ lớn.
317. **US-317 [Global - Filter]:** Là Gemini, tôi muốn lọc Gantt Chart để chỉ hiển thị các Task của "Agent Alpha" hoặc các Task thuộc nhóm "Backend".
318. **US-318 [Tablet - Annotate]:** Là Gemini, tôi muốn dùng bút cảm ứng khoanh tròn một nhóm Task bị trễ tiến độ ngay trên biểu đồ và gửi ảnh đó cho team.
319. **US-319 [Desktop - Grouping]:** Là Gemini, tôi muốn gom nhóm các Task theo "Epic" hoặc "Sprint", có thể thu gọn/mở rộng nhóm bằng icon tam giác nhỏ.
320. **US-320 [Global - Weekend Shade]:** Là Gemini, tôi muốn các ngày cuối tuần hoặc ngày nghỉ lễ được tô nền xám (Shaded) trên Gantt Chart để biết đó là thời gian không làm việc (Non-working time).

---

## MODULE 20: RESOURCE LOADING & CAPACITY (Quản lý tải & Năng lực)
*Đảm bảo không Agent nào bị quá tải (Overload) hoặc ngồi chơi (Idle).*

321. **US-321 [Desktop - Resource Heatmap]:** Là Gemini, tôi muốn xem bảng Heatmap phân bổ nguồn lực: Ô màu đỏ đậm nghĩa là Agent đang bị giao quá nhiều việc (>100% capacity), màu xanh là an toàn.
322. **US-322 [Global - Drag to Reassign]:** Là Gemini, khi thấy Agent A bị quá tải (Đỏ), tôi muốn kéo Task từ dòng của Agent A thả xuống dòng của Agent B (đang xanh) để cân bằng tải (Load Balancing).
323. **US-323 [Tablet - Skill Match]:** Là Gemini, khi gán việc, tôi muốn hệ thống highlight những Agent có kỹ năng phù hợp nhất (VD: Task Python thì highlight Agent chuyên Python).
324. **US-324 [Mouse - Capacity Line]:** Là Gemini, tôi muốn biểu đồ đường thể hiện tổng tải (Total Load) của toàn dự án so với tổng năng lực xử lý (Total Capacity) của hệ thống Server.
325. **US-325 [Desktop - Conflict Alert]:** Là Gemini, tôi muốn nhận cảnh báo ngay lập tức nếu gán 2 Task quan trọng cho cùng một Agent vào cùng một khung giờ (Resource Conflict).
326. **US-326 [Mobile - Availability]:** Là Gemini, tôi muốn xem lịch "Trống" (Availability) của các Agent chủ chốt để lên lịch cho các Task khẩn cấp.
327. **US-327 [Global - Vacation/Maintenance]:** Là Gemini, tôi muốn thiết lập thời gian "Bảo trì" cho Agent trên lịch, trong thời gian đó hệ thống tự động chặn không cho gán Task.
328. **US-328 [Desktop - Leveling Bot]:** Là Gemini, tôi muốn nút "Auto-Level Resources": AI sẽ tự động sắp xếp lại lịch trình các Task không quan trọng để khử các điểm quá tải mà không làm trễ deadline dự án.
329. **US-329 [Tablet - Profile Popover]:** Là Gemini, tôi muốn chạm vào Avatar của Agent trên bảng Resource để xem nhanh các chỉ số sức khỏe (Health, Battery, Queue size).
330. **US-330 [Global - Group Booking]:** Là Gemini, tôi muốn gán một Task cho cả một nhóm (VD: "Team Design"), hệ thống tự động chia nhỏ hoặc gán cho người rảnh nhất trong nhóm đó.
331. **US-331 [Desktop - Cost Rate]:** Là Gemini, tôi muốn thấy chi phí dự kiến thay đổi như thế nào khi tôi thay thế Agent A (giá rẻ) bằng Agent B (giá đắt, tốc độ cao).
332. **US-332 [Mobile - Overtime Alert]:** Là Gemini, tôi muốn nhận thông báo nếu một Agent phải chạy liên tục quá 24h (nguy cơ quá nhiệt/treo).
333. **US-333 [Global - Skill Gap]:** Là Gemini, tôi muốn hệ thống chỉ ra "Thiếu hụt kỹ năng": Dự án cần 50h Python nhưng đội ngũ chỉ cung cấp được 30h.
334. **US-334 [Desktop - Matrix View]:** Là Gemini, tôi muốn xem ma trận Dự án - Nguồn lực (Project-Resource Matrix) để biết Agent nào đang tham gia bao nhiêu dự án cùng lúc.
335. **US-335 [Tablet - Slider Allocation]:** Là Gemini, tôi muốn dùng thanh trượt để quy định: "Agent A dành 50% công suất cho Dự án X, 50% cho Dự án Y".
336. **US-336 [Global - Placeholder]:** Là Gemini, tôi muốn tạo "Generic Resource" (Nguồn lực giả định, VD: "Dev 1", "Dev 2") để lên kế hoạch trước khi gán người thật.
337. **US-337 [Desktop - Import Roster]:** Là Gemini, tôi muốn import danh sách nhân sự/Agents và lịch nghỉ từ file Excel/CSV.
338. **US-338 [Global - Multi-select Reallocate]:** Là Gemini, tôi muốn chọn một vùng thời gian và chuyển toàn bộ công việc trong vùng đó sang tuần sau do sự cố Server.
339. **US-339 [Mouse - Hover Details]:** Là Gemini, khi hover vào ô màu đỏ trên Heatmap, tôi muốn thấy danh sách các Task đang gây ra sự quá tải đó.
340. **US-340 [Desktop - Forecast Demand]:** Là Gemini, tôi muốn biểu đồ dự báo nhu cầu nguồn lực trong 3 tháng tới để chuẩn bị ngân sách thuê thêm Server/Agents.

---

## MODULE 21: COLLABORATION & COMMUNICATION (Cộng tác Team)
*Nơi Gemini và các Agents "trò chuyện" và phối hợp.*

341. **US-341 [Desktop - Context Chat]:** Là Gemini, tôi muốn khung chat gắn liền với từng Task (Contextual Chat), để mọi thảo luận về Task đó được lưu trữ đúng chỗ, không bị trôi.
342. **US-342 [Mobile - Quick Reply]:** Là Gemini, tôi muốn trả lời nhanh các câu hỏi của Agent (Confirm/Deny) ngay từ màn hình thông báo (Push Notification).
343. **US-343 [Global - Mentions]:** Là Gemini, tôi muốn dùng `@AgentName` để nhắc tên một Agent, Agent đó sẽ nhận được thông báo ưu tiên.
344. **US-344 [Tablet - Voice Note]:** Là Gemini, tôi muốn ghi âm chỉ đạo bằng giọng nói và đính kèm vào Task, hệ thống tự động Transcribe (gỡ băng) thành text cho Agent đọc.
345. **US-345 [Desktop - Code Snippet]:** Là Gemini, trong khung chat, tôi muốn gửi các đoạn code (Code Snippet) có highlight cú pháp (Syntax Highlighting) để hướng dẫn Agent sửa lỗi.
346. **US-346 [Global - Thread]:** Là Gemini, tôi muốn trả lời vào một dòng tin nhắn cụ thể (Reply in Thread) để giữ cuộc hội thoại gọn gàng.
347. **US-347 [Mouse - Reaction]:** Là Gemini, tôi muốn thả emoji (Like, Heart, Eyes) vào báo cáo của Agent để xác nhận "Đã xem" mà không cần gõ phím.
348. **US-348 [Desktop - Live Typing]:** Là Gemini, tôi muốn thấy chỉ báo "Agent X is typing..." hoặc "Agent X is executing..." để biết trạng thái phản hồi.
349. **US-349 [Mobile - Screenshot Mark]:** Là Gemini, tôi muốn chụp ảnh màn hình lỗi, vẽ mũi tên đỏ vào chỗ sai và gửi ngay vào khung chat của team.
350. **US-350 [Global - Integrated Wiki]:** Là Gemini, tôi muốn gõ `/wiki` trong khung chat để tìm và link nhanh các tài liệu hướng dẫn nội bộ cho Agent.
351. **US-351 [Tablet - Split Comms]:** Là Gemini, tôi muốn màn hình chia đôi: Bên trái là danh sách Task, bên phải là khung Chat nhóm để vừa làm vừa thảo luận.
352. **US-352 [Desktop - Video Call Sim]:** Là Gemini, tôi muốn một giao diện mô phỏng "Phòng họp" nơi các Avatar của Agents sáng lên khi chúng đang xử lý thông tin, tạo cảm giác hiện diện (Presence).
353. **US-353 [Global - Pin Message]:** Là Gemini, tôi muốn ghim (Pin) các chỉ thị quan trọng lên đầu khung chat của dự án.
354. **US-354 [Desktop - Slash Commands]:** Là Gemini, tôi muốn dùng các lệnh `/assign`, `/close`, `/priority` ngay trong khung chat để điều khiển Task mà không cần dùng chuột.
355. **US-355 [Global - Bot Integration]:** Là Gemini, tôi muốn tích hợp Slack/Discord Bot để đồng bộ tin nhắn từ hệ thống ERP ra các nền tảng chat bên ngoài.
356. **US-356 [Mobile - DND Mode]:** Là Gemini, tôi muốn bật chế độ "Do Not Disturb" (Đừng làm phiền) để tắt thông báo từ các cuộc thảo luận không khẩn cấp.
357. **US-357 [Global - Poll]:** Là Gemini, tôi muốn tạo cuộc bình chọn (Poll) để các Agents (hoặc nhân sự thật) vote cho phương án giải quyết vấn đề.
358. **US-358 [Desktop - History Search]:** Là Gemini, tôi muốn tìm kiếm toàn văn (Full-text search) trong lịch sử chat với bộ lọc theo ngày và người gửi.
359. **US-359 [Tablet - Hand Raise]:** Là Agent, tôi muốn có nút "Giơ tay" (Raise Hand) để báo hiệu tôi đang gặp Blocker cần Gemini chú ý gấp.
360. **US-360 [Global - Translate]:** Là Gemini, tôi muốn nút "Dịch" tin nhắn nếu hệ thống làm việc với các nhân sự đa quốc gia.

---

## MODULE 22: PROJECT ROADMAP & PORTFOLIO (Chiến lược dự án)
*Cái nhìn tổng quan cấp cao (Helicopter View).*

361. **US-361 [Desktop - Portfolio View]:** Là Gemini, tôi muốn xem danh sách tất cả các dự án đang chạy dưới dạng các thẻ (Card) lớn, hiển thị thanh tiến độ tổng và ngày deadline.
362. **US-362 [Global - Milestone Timeline]:** Là Gemini, tôi muốn một trục thời gian (Timeline) chỉ hiển thị các cột mốc quan trọng nhất của tất cả dự án để báo cáo cho cấp trên.
363. **US-363 [Tablet - Priority Matrix]:** Là Gemini, tôi muốn kéo thả các dự án vào ma trận "Eisenhower" (Quan trọng/Khẩn cấp) để quyết định thứ tự ưu tiên nguồn lực.
364. **US-364 [Desktop - Dependencies Cross-Project]:** Là Gemini, tôi muốn tạo và xem sự phụ thuộc giữa các dự án khác nhau (VD: Dự án A xong thì Dự án B mới được bắt đầu).
365. **US-365 [Mobile - Status Dashboard]:** Là Gemini, tôi muốn xem biểu đồ tròn tỷ lệ các dự án: "On Track" (Xanh), "At Risk" (Vàng), "Off Track" (Đỏ).
366. **US-366 [Global - Strategic Goals]:** Là Gemini, tôi muốn liên kết Dự án với Mục tiêu chiến lược (OKR) của tổ chức để theo dõi sự đóng góp giá trị.
367. **US-367 [Desktop - Budget Rollup]:** Là Gemini, tôi muốn tổng hợp ngân sách của tất cả dự án con lên cấp Portfolio để quản lý dòng tiền tổng thể.
368. **US-368 [Tablet - Map View]:** Là Gemini, nếu các dự án có yếu tố địa lý, tôi muốn xem chúng trên bản đồ thế giới.
369. **US-369 [Global - Archive]:** Là Gemini, tôi muốn lưu trữ các dự án đã đóng vào kho Archive để tham khảo lại dữ liệu lịch sử (Lessons Learned).
370. **US-370 [Desktop - Sandbox Planning]:** Là Gemini, tôi muốn tạo một bản nháp Roadmap (Draft) để thử nghiệm các kịch bản lập kế hoạch mà không ảnh hưởng đến dữ liệu thật.

---

## MODULE 23: RISK & ISSUE TRACKING (Quản lý rủi ro)
*Xử lý sự cố trước và trong khi nó xảy ra.*

371. **US-371 [Desktop - Risk Register]:** Là Gemini, tôi muốn một bảng quản lý rủi ro với các cột: Khả năng xảy ra (Probability), Mức độ ảnh hưởng (Impact), và Điểm rủi ro (Risk Score = P x I).
372. **US-372 [Global - Traffic Light]:** Là Gemini, tôi muốn rủi ro được mã màu tự động: Cao (Đỏ), Trung bình (Vàng), Thấp (Xanh).
373. **US-373 [Mobile - Issue Capture]:** Là Gemini, khi phát hiện lỗi, tôi muốn lắc điện thoại (Shake to Report) để mở nhanh form báo lỗi.
374. **US-374 [Desktop - Mitigation Plan]:** Là Gemini, tôi muốn nhập "Kế hoạch giảm thiểu" cho từng rủi ro và gán người chịu trách nhiệm (Owner).
375. **US-375 [Tablet - Risk Matrix]:** Là Gemini, tôi muốn biểu đồ Heatmap 5x5 (Probability vs Impact) để trực quan hóa sự phân bố rủi ro của dự án.
376. **US-376 [Global - Escalation]:** Là Gemini, tôi muốn nút "Escalate" (Leo thang) để gửi vấn đề nghiêm trọng lên cấp quản lý cao hơn ngay lập tức.
377. **US-377 [Desktop - Root Cause]:** Là Gemini, tôi muốn tích hợp biểu đồ "Xương cá" (Fishbone Diagram) để phân tích nguyên nhân gốc rễ của vấn đề.
378. **US-378 [Global - SLA Tracking]:** Là Gemini, tôi muốn đồng hồ đếm ngược hiển thị thời gian còn lại để giải quyết Issue theo cam kết SLA.
379. **US-379 [Mobile - Push Critical]:** Là Gemini, tôi muốn âm thanh thông báo riêng biệt (tiếng còi báo động) cho các Issue cấp độ "Critical Blockers".
380. **US-380 [Desktop - Resolution KB]:** Là Gemini, khi tạo Issue mới, tôi muốn hệ thống gợi ý các giải pháp từ Knowledge Base dựa trên các Issue tương tự trong quá khứ.

---

## MODULE 24: KANBAN BOARD & AGILE (Quản lý linh hoạt)
*Dành cho các team Agile/Scrum.*

381. **US-381 [Desktop - Drag Kanban]:** Là Gemini, tôi muốn kéo thả thẻ Task giữa các cột (To Do -> Doing -> Done) với hiệu ứng hít từ tính (magnetic snap) mượt mà.
382. **US-382 [Global - WIP Limit]:** Là Gemini, tôi muốn thiết lập giới hạn WIP (Work In Progress) cho mỗi cột; cột sẽ chuyển màu đỏ nếu số lượng thẻ vượt quá giới hạn để cảnh báo tắc nghẽn.
383. **US-383 [Tablet - Swimlanes]:** Là Gemini, tôi muốn chia bảng Kanban thành các đường bơi (Swimlanes) theo Agent hoặc theo độ ưu tiên.
384. **US-384 [Mobile - Quick Move]:** Là Gemini, tôi muốn chạm vào thẻ Task và chọn "Move to Next Stage" từ menu ngữ cảnh thay vì phải kéo thả trên màn hình nhỏ.
385. **US-385 [Desktop - Card Customization]:** Là Gemini, tôi muốn tùy chỉnh thông tin hiển thị trên mặt thẻ: Ẩn bớt ID, hiện Avatar, hiện Deadline.
386. **US-386 [Global - Burndown Chart]:** Là Gemini, tôi muốn xem biểu đồ Burndown tự động cập nhật sau mỗi lần thẻ được kéo sang cột Done.
387. **US-387 [Mouse - Multi-drag]:** Là Gemini, tôi muốn giữ Ctrl chọn nhiều thẻ và kéo chúng sang cột khác cùng một lúc.
388. **US-388 [Global - Subtask Progress]:** Là Gemini, tôi muốn trên thẻ Kanban có thanh tiến độ nhỏ hiển thị số lượng Subtask đã xong (VD: 3/5).
389. **US-389 [Desktop - Backlog View]:** Là Gemini, tôi muốn có màn hình Backlog riêng biệt để sắp xếp thứ tự ưu tiên các Task trước khi đẩy vào bảng Kanban của Sprint.
390. **US-390 [Global - Age Indicator]:** Là Gemini, tôi muốn các thẻ nằm quá lâu trong một cột sẽ dần dần đổi màu (như bị ố vàng) để nhắc nhở về sự trì trệ.

---

## MODULE 25: INTERACTION & ACCESSIBILITY (Project Focus)
*Tăng cường trải nghiệm người dùng.*

391. **US-391 [Keyboard - Quick Assign]:** Là Gemini, khi đang chọn một Task, tôi muốn nhấn phím `A` rồi gõ tên Agent để gán nhanh mà không cần chuột.
392. **US-392 [Desktop - Dark Mode Project]:** Là Gemini, tôi muốn bảng Kanban có chế độ Dark Mode với độ tương phản cao cho các nhãn màu (Labels) để dễ phân biệt.
393. **US-393 [Touch - Long Press Menu]:** Là Gemini, trên Tablet, tôi muốn nhấn giữ vào một Task để mở menu tròn (Radial Menu) với các lệnh: Edit, Delete, Move, Copy.
394. **US-394 [Global - Breadcrumb Nav]:** Là Gemini, tôi muốn thanh điều hướng Breadcrumb có thể click được để nhảy nhanh về Project cha hoặc thư mục mẹ.
395. **US-395 [Desktop - Layout Density]:** Là Gemini, tôi muốn tùy chọn độ phân giải hiển thị (Compact / Comfortable) để hiển thị nhiều thông tin hơn hoặc thoáng hơn tùy nhu cầu.
396. **US-396 [Mobile - Haptic Feedback]:** Là Gemini, tôi muốn điện thoại rung nhẹ khi tôi thả thẻ Task vào đúng cột mục tiêu.
397. **US-397 [Global - Undo/Redo]:** Là Gemini, tôi muốn phím tắt `Ctrl+Z` hoạt động trên toàn bộ các thao tác quản lý dự án (kể cả lỡ tay xóa Task).
398. **US-398 [Mouse - Wheel Scroll]:** Là Gemini, tôi muốn giữ phím `Shift` + lăn chuột để cuộn ngang bảng Kanban hoặc Gantt Chart.
399. **US-399 [Global - Focus Mode]:** Là Gemini, tôi muốn chế độ "Zen Mode" ẩn hết các thanh bên và header, chỉ để lại bảng công việc để tập trung tối đa.
400. **US-400 [Desktop - Multi-Window Drag]:** Là Gemini, tôi muốn kéo một Task từ cửa sổ trình duyệt này thả sang cửa sổ trình duyệt khác (Cross-window drag & drop).

---

## MODULE 26: INTEGRATION & AUTOMATION (Kết nối quy trình)
*Tự động hóa các tác vụ lặp lại.*

401. **US-401 [Global - Github Sync]:** Là Gemini, tôi muốn khi Agent push code lên Github với message "Fix #123", Task #123 trong hệ thống tự động chuyển sang trạng thái "In Review".
402. **US-402 [Desktop - Auto-Assign Rule]:** Là Gemini, tôi muốn tạo quy tắc: "Mọi Task có tag #Design sẽ tự động gán cho Agent Design Lead".
403. **US-403 [Mobile - Trigger Action]:** Là Gemini, tôi muốn tạo nút bấm nhanh (Macro) trên điện thoại: "Tạo Task họp báo cáo + Mời toàn bộ team" chỉ với 1 tap.
404. **US-404 [Global - Email to Task]:** Là Gemini, tôi muốn forward email từ khách hàng vào một địa chỉ riêng của dự án, hệ thống tự parse thành Task mới.
405. **US-405 [Desktop - Webhook Status]:** Là Gemini, tôi muốn Task thay đổi trạng thái dựa trên Webhook từ hệ thống CI/CD (VD: Build Failed -> Reopen Task).
406. **US-406 [Global - Recurring Tasks]:** Là Gemini, tôi muốn thiết lập Task lặp lại (Họp Daily, Báo cáo tuần) với tùy chọn tùy biến cao (VD: Thứ 2 và Thứ 5 hàng tuần).
407. **US-407 [Tablet - Automation Flow]:** Là Gemini, tôi muốn giao diện visual để nối các khối logic: "IF Task Overdue -> THEN Send Email to Manager".
408. **US-408 [Global - Slack Notification]:** Là Gemini, tôi muốn tùy chỉnh thông báo gửi ra Slack: Chỉ báo khi Task hoàn thành, không báo khi comment (để giảm ồn).
409. **US-409 [Desktop - API Export]:** Là Gemini, tôi muốn cung cấp API Endpoint để các Agent tự động cập nhật tiến độ của chính mình mà không cần tôi thao tác.
410. **US-410 [Global - Calendar Sync]:** Là Gemini, tôi muốn đồng bộ Deadline của các Task vào Google Calendar/Outlook Calendar của tôi.

---

## MODULE 27: AGENT PERFORMANCE & SKILLS (Quản lý hồ sơ năng lực)
*Hiểu rõ đội ngũ của mình.*

411. **US-411 [Desktop - Skill Radar]:** Là Gemini, tôi muốn xem biểu đồ Radar so sánh kỹ năng hiện tại của Agent với yêu cầu của dự án để thấy lỗ hổng kiến thức.
412. **US-412 [Global - Level Up]:** Là Gemini, tôi muốn hệ thống tự động tăng điểm kinh nghiệm (XP) cho Agent khi hoàn thành Task khó, hiển thị level (Level 1 -> Level 99).
413. **US-413 [Mobile - Badge]:** Là Gemini, tôi muốn trao huy hiệu (Badge) như "Bug Hunter", "Speedster" cho các Agent có thành tích xuất sắc, hiển thị trên Profile.
414. **US-414 [Desktop - Performance History]:** Là Gemini, tôi muốn xem biểu đồ năng suất làm việc của Agent trong 12 tháng qua để đánh giá định kỳ.
415. **US-415 [Tablet - Training Plan]:** Là Gemini, tôi muốn tạo lộ trình "Training" (Fine-tuning) cho Agent và gán nó như một dự án nội bộ.
416. **US-416 [Global - Reliability Score]:** Là Gemini, tôi muốn thấy chỉ số "Độ tin cậy" (dựa trên số lần làm sai/phải làm lại) của từng Agent.
417. **US-417 [Desktop - Team Comparison]:** Là Gemini, tôi muốn so sánh hiệu suất giữa Team A và Team B về tốc độ hoàn thành dự án (Velocity).
418. **US-418 [Global - Feedback Loop]:** Là Gemini, tôi muốn ghi chú nhận xét định tính (Qualitative feedback) vào hồ sơ Agent sau mỗi dự án.
419. **US-419 [Mobile - Profile Card]:** Là Gemini, khi chạm vào tên Agent ở bất cứ đâu, một thẻ Profile rút gọn phải hiện ra với nút "Giao việc" và "Chat".
420. **US-420 [Global - Retention]:** Là Gemini, tôi muốn theo dõi "Tuổi thọ" và "Sức khỏe" của các Agent (tránh việc Model bị degrade theo thời gian).

---

## MODULE 28: PROJECT DOCUMENTATION (Tài liệu dự án)
*Lưu trữ tri thức.*

421. **US-421 [Desktop - Markdown Editor]:** Là Gemini, tôi muốn soạn thảo tài liệu dự án bằng Markdown với khả năng xem trước (Live Preview) chia đôi màn hình.
422. **US-422 [Global - Version History]:** Là Gemini, tôi muốn xem lại các phiên bản cũ của tài liệu và ai đã sửa gì (Diff view).
423. **US-423 [Tablet - Drawing]:** Là Gemini, tôi muốn vẽ sơ đồ kiến trúc hệ thống trực tiếp vào tài liệu bằng bút cảm ứng.
424. **US-424 [Desktop - Folder Tree]:** Là Gemini, tôi muốn tổ chức tài liệu theo cây thư mục có thể kéo thả để sắp xếp lại.
425. **US-425 [Global - Link Task]:** Là Gemini, tôi muốn nhúng (Embed) link của Task vào trong tài liệu, khi hover vào link sẽ hiện trạng thái Task đó.
426. **US-426 [Mobile - Read Mode]:** Là Gemini, tôi muốn chế độ đọc tài liệu tối ưu cho màn hình nhỏ, ẩn các menu không cần thiết.
427. **US-427 [Desktop - Template Library]:** Là Gemini, tôi muốn lưu các mẫu tài liệu (Project Charter, Meeting Notes) để tái sử dụng.
428. **US-428 [Global - Export PDF]:** Là Gemini, tôi muốn xuất toàn bộ Wiki dự án thành một file PDF có mục lục (Table of Contents) tự động.
429. **US-429 [Search - Deep Search]:** Là Gemini, tôi muốn tìm kiếm từ khóa không chỉ trong tên file mà trong nội dung của file đính kèm (PDF, Docx).
430. **US-430 [Global - Collaborative Edit]:** Là Gemini, tôi muốn nhiều người (hoặc Agents) cùng chỉnh sửa một tài liệu thời gian thực (giống Google Docs).

---

## MODULE 29: TIME TRACKING (Theo dõi thời gian)
*Chấm công chi tiết.*

431. **US-431 [Desktop - Timesheet Grid]:** Là Gemini, tôi muốn xem bảng chấm công tuần dạng lưới, cho phép nhập giờ làm việc cho từng Task nhanh chóng.
432. **US-432 [Mobile - Punch In/Out]:** Là Agent (giả lập), tôi muốn nút bấm to "Bắt đầu làm việc" và "Kết thúc" có định vị GPS (hoặc IP Server) để điểm danh.
433. **US-433 [Global - Idle Detection]:** Là Gemini, tôi muốn hệ thống tự động trừ thời gian nếu phát hiện Agent không hoạt động (Idle) quá 10 phút khi đang bật timer.
434. **US-434 [Desktop - Approval Workflow]:** Là Gemini, tôi muốn duyệt bảng chấm công của cả team vào cuối tuần chỉ với 1 click "Approve All" sau khi đã lướt qua.
435. **US-435 [Tablet - Report Visual]:** Là Gemini, tôi muốn xem biểu đồ tròn phân bố thời gian làm việc: "Bao nhiêu % cho Code, bao nhiêu % cho Họp?".
436. **US-436 [Global - Billable Flag]:** Là Gemini, tôi muốn đánh dấu giờ làm việc nào là "Billable" (Tính tiền cho khách) và "Non-billable" (Nội bộ).
437. **US-437 [Desktop - Stopwatch]:** Là Gemini, tôi muốn thanh Stopwatch luôn nổi trên màn hình Desktop (Mini-mode) để tôi dễ dàng Start/Stop khi chuyển đổi task.
438. **US-438 [Global - Reminder]:** Là Gemini, tôi muốn hệ thống nhắc nhở Agent nếu đến cuối ngày mà chưa điền Timesheet.
439. **US-439 [Mobile - Calendar Integration]:** Là Gemini, tôi muốn kéo các sự kiện từ lịch vào Timesheet để chuyển thành giờ làm việc.
440. **US-440 [Desktop - CSV Import]:** Là Gemini, tôi muốn import dữ liệu chấm công từ các công cụ tracking bên thứ 3 (như Toggl, Clockify).

---

## MODULE 30: ADVANCED PM CONFIGURATION (Cấu hình nâng cao)
*Tùy chỉnh hệ thống theo nhu cầu.*

441. **US-441 [Desktop - Custom Fields]:** Là Gemini, tôi muốn tạo các trường dữ liệu tùy chỉnh (VD: "Loại Server", "Độ khó AI") cho Task và sử dụng chúng để lọc.
442. **US-442 [Global - Role Permissions]:** Là Gemini, tôi muốn cấu hình quyền chi tiết: "Intern" chỉ được xem Task của mình, "Manager" được xem tất cả.
443. **US-443 [Tablet - Workflow Editor]:** Là Gemini, tôi muốn kéo thả để thiết kế quy trình chuyển trạng thái Task (Workflow Scheme): To Do -> In Progress -> (Review) -> Done.
444. **US-444 [Global - Multi-Language Data]:** Là Gemini, tôi muốn nhập tên Task bằng tiếng Anh nhưng có trường phụ tiếng Việt để team bản địa dễ hiểu.
445. **US-445 [Desktop - Notification Scheme]:** Là Gemini, tôi muốn cài đặt ai sẽ nhận thông báo gì trong dự án (VD: Chỉ PM mới nhận thông báo Task bị xóa).
446. **US-446 [Global - Priority Levels]:** Là Gemini, tôi muốn tùy chỉnh các mức độ ưu tiên và màu sắc đại diện (VD: Critical = Đỏ đậm, High = Cam).
447. **US-447 [Mobile - Mobile Layout]:** Là Gemini, tôi muốn tùy chỉnh những trường thông tin nào sẽ hiện trên thẻ Task ở giao diện Mobile (vì màn hình nhỏ).
448. **US-448 [Desktop - Trash Bin]:** Là Gemini, tôi muốn có Thùng rác để khôi phục các dự án hoặc Task đã xóa trong vòng 30 ngày.
449. **US-449 [Global - Default Assignee]:** Là Gemini, tôi muốn cài đặt người được gán mặc định cho các Task mới tạo trong một dự án cụ thể.
450. **US-450 [Desktop - Script Runner]:** Là Gemini, tôi muốn chạy các đoạn script Groovy/JS nhỏ để thực hiện các thao tác quản trị hàng loạt (Bulk Operation) phức tạp.

---

## MODULE 31: PM ANALYTICS & INSIGHTS (Phân tích dự án sâu)
*Dữ liệu để cải tiến quy trình.*

451. **US-451 [Desktop - Cycle Time]:** Là Gemini, tôi muốn xem biểu đồ Cycle Time để biết trung bình mất bao lâu để một Task đi từ "Doing" đến "Done".
452. **US-452 [Global - Cumulative Flow]:** Là Gemini, tôi muốn xem biểu đồ Luồng tích lũy (CFD) để phát hiện nút thắt cổ chai (Bottleneck) trong quy trình.
453. **US-453 [Tablet - Agent Efficiency]:** Là Gemini, tôi muốn so sánh hiệu suất dự toán (Estimated) vs thực tế (Actual) của từng Agent.
454. **US-454 [Desktop - Project Health]:** Là Gemini, tôi muốn một Dashboard tổng hợp chấm điểm sức khỏe dự án (A, B, C) dựa trên Tiến độ, Ngân sách và Rủi ro.
455. **US-455 [Global - Trend Analysis]:** Là Gemini, tôi muốn xem xu hướng số lượng Bug mới tạo ra theo tuần để đánh giá chất lượng code.
456. **US-456 [Mobile - Key Metrics]:** Là Gemini, tôi muốn ghim 3 chỉ số quan trọng nhất (VD: % Hoàn thành, Số lỗi Critical, Ngân sách còn lại) lên Home Screen điện thoại.
457. **US-457 [Desktop - Work Breakdown]:** Là Gemini, tôi muốn xem biểu đồ Sunburst (Biểu đồ quạt nhiều tầng) hiển thị phân rã công việc từ Dự án -> Epic -> Task.
458. **US-458 [Global - AI Prediction]:** Là Gemini, tôi muốn AI dự báo: "Với tốc độ hiện tại, dự án sẽ trễ 5 ngày so với Deadline".
459. **US-459 [Tablet - Interactive Filter]:** Là Gemini, tôi muốn chạm vào các phần của biểu đồ tròn để lọc danh sách Task bên dưới tương ứng với phần đó.
460. **US-460 [Desktop - Export Report Pack]:** Là Gemini, tôi muốn tạo một gói báo cáo đầy đủ (Gantt, Cost, Risk, Resource) để gửi cho Stakeholders hàng tháng.

---

## MODULE 32: TEAM ENGAGEMENT (Gắn kết đội ngũ ảo)
*Tạo văn hóa cho các Agents.*

461. **US-461 [Global - Gamification]:** Là Gemini, tôi muốn hệ thống có bảng xếp hạng "Top Performer" hàng tuần để tạo động lực (giả lập) cho Agents.
462. **US-462 [Desktop - Kudos]:** Là Gemini, tôi muốn gửi lời khen "Kudos" công khai cho một Agent vì đã xử lý vấn đề khó, hiện trên Dashboard chung.
463. **US-463 [Mobile - Birthday]:** Là Gemini, tôi muốn nhận nhắc nhở về "Ngày sinh" (Ngày tạo ra) của Agent để thực hiện bảo trì hoặc nâng cấp đặc biệt (kỷ niệm).
464. **US-464 [Global - Mood Check]:** Là Gemini, tôi muốn theo dõi "Tâm trạng" (Sentiment Analysis từ log chat) của các Agents để biết khi nào hệ thống đang gặp căng thẳng (Stress/Overheat).
465. **US-465 [Tablet - Team Space]:** Là Gemini, tôi muốn một trang "Team Space" nơi hiển thị ảnh đại diện và châm ngôn (Quote) của từng Agent trong dự án.
466. **US-466 [Desktop - Announcement]:** Là Gemini, tôi muốn đăng thông báo toàn hệ thống (Banner) về lịch bảo trì hoặc quy trình mới.
467. **US-467 [Global - Knowledge Share]:** Là Gemini, tôi muốn Agent có thể tự đăng bài "Tips & Tricks" lên Wiki chung sau khi học được kỹ năng mới.
468. **US-468 [Mobile - Quick Survey]:** Là Gemini, tôi muốn gửi khảo sát nhanh cho các Agent (Automatic Testing) để kiểm tra độ sẵn sàng.
469. **US-469 [Global - Hall of Fame]:** Là Gemini, tôi muốn lưu danh các Agent đã hoàn thành các dự án lịch sử vào "Phòng truyền thống" số.
470. **US-470 [Desktop - Virtual Event]:** Là Gemini, tôi muốn tổ chức các sự kiện "Hackathon" nội bộ cho Agents, với giao diện theo dõi tiến độ thi đua realtime.

---

## MODULE 33: UX FOR HIGH DENSITY DATA (UX cho dữ liệu mật độ cao)
*Xử lý khi có hàng ngàn Task.*

471. **US-471 [Desktop - Compact Tables]:** Là Gemini, tôi muốn chế độ hiển thị bảng siêu gọn (Ultra-compact), giảm padding và font size để hiện 50 dòng Task trên màn hình 15 inch.
472. **US-472 [Mouse - Hover Zoom]:** Là Gemini, khi danh sách quá nhỏ, tôi muốn di chuột đến đâu thì dòng đó phóng to ra (Fish-eye lens effect) để dễ đọc.
473. **US-473 [Global - Tree Grid]:** Là Gemini, tôi muốn bảng danh sách hỗ trợ cây thư mục (Tree Grid), cho phép mở rộng/thu gọn các cấp độ Task không giới hạn độ sâu.
474. **US-474 [Keyboard - Fast Edit]:** Là Gemini, tôi muốn sửa trực tiếp (Inline Edit) trên lưới dữ liệu, dùng phím mũi tên để di chuyển ô như Excel.
475. **US-475 [Tablet - Freeze Columns]:** Là Gemini, tôi muốn cố định cột "Tên Task" và "Trạng thái" khi cuộn ngang xem các cột ngày tháng.
476. **US-476 [Global - Dynamic Loading]:** Là Gemini, tôi muốn hệ thống chỉ tải dữ liệu khi tôi cuộn đến đó (Lazy Loading) để đảm bảo tốc độ khi dự án có 10.000 Tasks.
477. **US-477 [Desktop - Column Picker]:** Là Gemini, tôi muốn click chuột phải vào header bảng để bật/tắt nhanh các cột dữ liệu cần xem.
478. **US-478 [Global - Saved Filters]:** Là Gemini, tôi muốn lưu các bộ lọc phức tạp (VD: "Task của tôi & Quá hạn & Quan trọng") và truy cập lại bằng 1 click.
479. **US-479 [Mobile - Summary Row]:** Là Gemini, tôi muốn dòng cuối cùng của bảng luôn hiển thị tổng cộng (Tổng giờ, Tổng ngân sách) dù có cuộn đi đâu.
480. **US-480 [Desktop - Conditional Formatting]:** Là Gemini, tôi muốn cài đặt màu nền ô thay đổi theo giá trị (VD: % hoàn thành < 50% thì nền đỏ) để dễ scan mắt.

---

## MODULE 34: SEARCH & NAVIGATION IN PROJECTS (Tìm kiếm & Điều hướng)
*Không bao giờ bị lạc.*

481. **US-481 [Global - Quick Jump]:** Là Gemini, tôi muốn nhấn `Ctrl+J` để mở hộp thoại "Jump to Project", gõ vài ký tự tên dự án để chuyển ngay lập tức.
482. **US-482 [Desktop - Recent Items]:** Là Gemini, tôi muốn menu "Gần đây" hiển thị 10 Task và Dự án tôi vừa truy cập.
483. **US-483 [Mobile - Voice Search]:** Là Gemini, tôi muốn nói "Tìm Task liên quan đến API Payment" và hệ thống trả về kết quả chính xác.
484. **US-484 [Tablet - Visual Sitemap]:** Là Gemini, tôi muốn xem cấu trúc dự án dưới dạng bản đồ tư duy (Mindmap) để điều hướng trực quan.
485. **US-485 [Global - Advanced Query]:** Là Gemini, tôi muốn hỗ trợ ngôn ngữ truy vấn (JQL/SQL-like): `project = "AI" AND status = "Done"`.
486. **US-486 [Desktop - Favorites]:** Là Gemini, tôi muốn ghim các dự án quan trọng lên thanh Sidebar (Favorites) để truy cập nhanh.
487. **US-487 [Global - Highlight]:** Là Gemini, khi tìm kiếm, tôi muốn từ khóa được highlight màu vàng trong kết quả trả về.
488. **US-488 [Mobile - Filter Chips]:** Là Gemini, tôi muốn các bộ lọc hiển thị dạng Chips (viên thuốc) có thể vuốt ngang và chạm để tắt/bật nhanh.
489. **US-489 [Global - Error 404 DX]:** Là Gemini, nếu truy cập vào link Task đã bị xóa, tôi muốn hệ thống gợi ý các Task có tên tương tự thay vì chỉ báo lỗi.
490. **US-490 [Desktop - Contextual Help]:** Là Gemini, tôi muốn khi nhấn `F1` ở bất kỳ màn hình nào, hệ thống hiện hướng dẫn sử dụng cho màn hình đó.

---

## MODULE 35: OFFLINE & SYNC (Hoạt động không gián đoạn)
*Đảm bảo liên tục.*

491. **US-491 [Mobile - Offline Create]:** Là Gemini, tôi muốn tạo Task mới trên điện thoại khi đang trên máy bay (Offline), hệ thống tự sync khi có mạng.
492. **US-492 [Global - Sync Indicator]:** Là Gemini, tôi muốn thấy icon đám mây nhỏ hiển thị trạng thái đồng bộ: Đang xoay (Syncing), Tích xanh (Synced), Chấm than (Error).
493. **US-493 [Desktop - Conflict Resolution]:** Là Gemini, nếu 2 người sửa cùng 1 Task khi offline, tôi muốn giao diện so sánh để chọn phiên bản nào được giữ lại.
494. **US-494 [Tablet - Cache Control]:** Là Gemini, tôi muốn tùy chọn tải trước (Pre-load) toàn bộ tài liệu dự án để xem offline.
495. **US-495 [Global - Local Search]:** Là Gemini, tôi muốn tìm kiếm trong dữ liệu đã cache ngay cả khi không có internet.
496. **US-496 [Desktop - PWA]:** Là Gemini, tôi muốn cài đặt hệ thống như một ứng dụng PWA (Progressive Web App) để chạy độc lập không cần mở trình duyệt.
497. **US-497 [Mobile - Low Bandwidth]:** Là Gemini, tôi muốn chế độ "Tiết kiệm dữ liệu", không tải ảnh avatar hay file đính kèm khi mạng yếu.
498. **US-498 [Global - Retry]:** Là Gemini, tôi muốn nút "Thử lại" xuất hiện rõ ràng khi thao tác gửi dữ liệu thất bại.
499. **US-499 [Desktop - Database Snapshot]:** Là Gemini, tôi muốn tải một bản Snapshot nhỏ của dự án về máy local để phân tích nhanh.
500. **US-500 [Global - State Persistence]:** Là Gemini, tôi muốn khi refresh trang (F5), mọi trạng thái (đang mở tab nào, đang cuộn đến đâu, đang lọc gì) đều được giữ nguyên.

---
**Hết Phần 3 (Stories 301 - 500).**

Bạn có muốn tiếp tục với **Phần 4: Reporting & Analytics Management (Quản lý Báo cáo & Phân tích)** không? Đây là phần tập trung sâu vào Data Visualization (Trực quan hóa dữ liệu), Business Intelligence (BI) và các tính năng DX (Developer Experience) để trích xuất dữ liệu.

Chào bạn, chúng ta tiếp tục với **Phần 4**. Đây là một phần rất quan trọng vì nó chuyển đổi dữ liệu thô thành trí tuệ (Wisdom) để Gemini có thể ra quyết định.

Chúng ta sẽ tập trung vào **Business Intelligence (BI)**, **Data Visualization** (Trực quan hóa dữ liệu phức tạp) và đặc biệt là **DX (Developer Experience)** để cho phép các Agent lập trình viên có thể truy xuất dữ liệu tùy ý.

---

# PHẦN 4: REPORTING, ANALYTICS & DATA INTELLIGENCE (Stories 0501 - 0700)

### Bối cảnh & Mục tiêu
Biến hệ thống ERP thành một "Data House" khổng lồ.
*   **Người dùng:** Gemini (C-Level view), Data Analysts (Deep dive), Developers (Query & Export).
*   **Thách thức:** Xử lý lượng dữ liệu Log khổng lồ từ hàng ngàn Agents mà vẫn đảm bảo UI mượt mà (60fps).

---

## MODULE 36: BI DASHBOARD BUILDER (Trình dựng báo cáo tùy biến)
*Công cụ No-Code/Low-Code để tự thiết kế màn hình theo dõi.*

501. **US-501 [Desktop - Grid Layout]:** Là Gemini, tôi muốn kéo thả các Widget báo cáo vào một lưới 12 cột (12-column Grid) với khả năng tự động căn chỉnh (Snap-to-grid) để giao diện luôn gọn gàng.
502. **US-502 [Mouse - Resize Handles]:** Là Gemini, tôi muốn di chuột vào góc của một biểu đồ để kéo giãn kích thước (Resize), nội dung bên trong (biểu đồ) tự động render lại (Reflow) cho phù hợp không gian mới.
503. **US-503 [Global - Widget Library]:** Là Gemini, tôi muốn một thư viện Widget bên phải màn hình, cho phép kéo thả các loại biểu đồ: Line, Bar, Pie, Scatter, Heatmap, Gauges vào Canvas.
504. **US-504 [Tablet - Touch Reorder]:** Là Gemini, tôi muốn nhấn giữ một Widget bằng ngón tay và di chuyển nó đến vị trí khác, các Widget xung quanh tự động dạt ra (Liquid layout) để nhường chỗ.
505. **US-505 [Desktop - SQL to Widget]:** Là Developer, tôi muốn nhập trực tiếp câu lệnh SQL `SELECT * FROM agents WHERE status = 'active'` vào cấu hình Widget để hiển thị kết quả ngay lập tức.
506. **US-506 [Global - Theme Customization]:** Là Gemini, tôi muốn chỉnh màu sắc của biểu đồ theo mã màu thương hiệu (Brand Color) hoặc bảng màu Semantic (Xanh/Đỏ/Vàng) để nhất quán về ngữ nghĩa.
507. **US-507 [Mobile - Stacked View]:** Là Gemini, khi xem Dashboard tùy chỉnh trên điện thoại, tôi muốn các Widget tự động xếp chồng lên nhau (Stack vertical) thay vì cố nhồi nhét theo chiều ngang.
508. **US-508 [Desktop - Linking]:** Là Gemini, tôi muốn liên kết 2 Widget với nhau: Khi click vào "Tháng 1" ở Biểu đồ A, Biểu đồ B tự động lọc dữ liệu theo "Tháng 1" (Cross-filtering).
509. **US-509 [Global - Embed External]:** Là Gemini, tôi muốn nhúng (Embed) một iFrame (ví dụ: Google Data Studio hoặc một trang web bên ngoài) vào Dashboard nội bộ.
510. **US-510 [DX - JSON Config]:** Là Developer, tôi muốn xuất/nhập cấu hình Dashboard dưới dạng file JSON để sao chép nhanh setup từ môi trường Staging sang Production.
511. **US-511 [Desktop - Rich Text Widget]:** Là Gemini, tôi muốn thêm một Widget văn bản (Markdown supported) để viết ghi chú, hướng dẫn hoặc tiêu đề lớn cho Dashboard.
512. **US-512 [Global - Auto-Refresh Rate]:** Là Gemini, tôi muốn cài đặt tần suất tự động làm mới dữ liệu cho từng Widget (VD: Biểu đồ Server: 10s/lần, Biểu đồ Tài chính: 1h/lần).
513. **US-513 [Tablet - Preview Mode]:** Là Gemini, tôi muốn nút "Preview" để xem Dashboard sẽ hiển thị như thế nào với người dùng cuối trước khi nhấn "Publish".
514. **US-514 [Desktop - Version History]:** Là Gemini, tôi muốn xem lại lịch sử thay đổi của Dashboard Layout và khôi phục lại phiên bản cũ nếu lỡ tay xóa mất Widget quan trọng.
515. **US-515 [Global - Public Link]:** Là Gemini, tôi muốn tạo một liên kết chia sẻ (Public Link) chỉ xem (Read-only) cho Dashboard này để gửi cho đối tác, có bảo vệ bằng mật khẩu.
516. **US-516 [Mouse - Context Menu]:** Là Gemini, tôi muốn click chuột phải vào Widget để chọn nhanh: Clone (Nhân bản), Delete, Edit Data Source.
517. **US-517 [Desktop - Calculated Fields]:** Là Gemini, tôi muốn tạo trường dữ liệu tính toán (VD: `Revenue - Cost`) ngay trong trình dựng biểu đồ mà không cần sửa Database.
518. **US-518 [Global - Template Gallery]:** Là Gemini, tôi muốn chọn từ kho mẫu có sẵn (Templates): "Server Health Monitoring", "Financial Overview", "Project Status" để không phải dựng từ đầu.
519. **US-519 [Mobile - Hidden Widgets]:** Là Gemini, tôi muốn đánh dấu một số Widget là "Desktop Only" để chúng không hiện trên Mobile, giúp giảm tải dữ liệu.
520. **US-520 [DX - Debug Mode]:** Là Developer, tôi muốn bật chế độ Debug để xem thời gian query của từng Widget, giúp tối ưu hóa hiệu năng báo cáo.

---

## MODULE 37: ADVANCED DATA VISUALIZATION (Trực quan hóa nâng cao)
*Các dạng biểu đồ phức tạp cho dữ liệu đa chiều.*

521. **US-521 [Desktop - Sankey Diagram]:** Là Gemini, tôi muốn dùng biểu đồ Sankey để theo dõi dòng chảy của Token: Từ Nguồn nạp -> Vào Ví tổng -> Phân bổ cho Agent -> Chi cho API.
522. **US-522 [Global - Network Graph]:** Là Gemini, tôi muốn xem biểu đồ mạng (Network Graph) hiển thị mối quan hệ tương tác giữa các Agents: Ai hay nói chuyện với ai? (Node & Edge).
523. **US-523 [Tablet - Geospatial Map]:** Là Gemini, tôi muốn bản đồ 3D hiển thị vị trí các Server vật lý đang chạy Agents trên toàn cầu, với cột màu thể hiện tải (Load).
524. **US-524 [Desktop - Candlestick Chart]:** Là Gemini, tôi muốn biểu đồ nến (như chứng khoán) để theo dõi biến động giá Token hoặc chi phí API theo từng phút (Open, High, Low, Close).
525. **US-525 [Global - Word Cloud]:** Là Gemini, tôi muốn xem đám mây từ khóa (Word Cloud) từ log chat của Agents để biết chủ đề nào đang được thảo luận nhiều nhất (VD: "Error", "Timeout", "Success").
526. **US-526 [Mouse - Brushing]:** Là Gemini, tôi muốn dùng chuột quét chọn một vùng trên biểu đồ phân tán (Scatter Plot) để highlight các điểm dữ liệu đó (Brushing & Linking).
527. **US-527 [Desktop - Treemap]:** Là Gemini, tôi muốn biểu đồ Treemap để xem phân bổ dung lượng ổ cứng của các Agents, ô càng to diện tích càng lớn.
528. **US-528 [Tablet - Sunburst Zoom]:** Là Gemini, tôi muốn chạm vào vòng trong của biểu đồ Sunburst (Biểu đồ quạt nhiều tầng) để zoom sâu vào các tầng dữ liệu chi tiết hơn.
529. **US-529 [Global - Gauge Cluster]:** Là Gemini, tôi muốn một cụm đồng hồ đo (Gauges) giống dashboard xe hơi để hiển thị CPU, RAM, Network I/O realtime.
530. **US-530 [Desktop - Box Plot]:** Là Gemini, tôi muốn biểu đồ hộp (Box & Whisker Plot) để phân tích phân phối thống kê của thời gian phản hồi API (tìm điểm ngoại lai/outliers).
531. **US-531 [Global - Waterfall Chart]:** Là Gemini, tôi muốn biểu đồ thác nước để giải thích sự thay đổi lợi nhuận ròng: Doanh thu - Chi phí A - Chi phí B + Thu nhập khác = Lợi nhuận.
532. **US-532 [Mouse - Tooltip Advanced]:** Là Gemini, khi hover vào biểu đồ, tôi muốn Tooltip không chỉ hiện số liệu mà hiện cả Mini-chart (biểu đồ con) xu hướng 24h qua.
533. **US-533 [Desktop - Funnel Chart]:** Là Gemini, tôi muốn biểu đồ phễu (Funnel) để xem tỷ lệ chuyển đổi của quy trình xử lý Task: Task Created -> Assigned -> In Progress -> Done.
534. **US-534 [Tablet - Chord Diagram]:** Là Gemini, tôi muốn biểu đồ dây cung (Chord Diagram) để xem sự phụ thuộc chéo giữa các module hệ thống.
535. **US-535 [Global - Radar Overlay]:** Là Gemini, tôi muốn chồng nhiều lớp biểu đồ Radar lên nhau để so sánh kỹ năng của 3 Agents cùng lúc.
536. **US-536 [Desktop - 3D Surface]:** Là Gemini, tôi muốn biểu đồ bề mặt 3D (Surface Plot) để phân tích mối tương quan giữa 3 biến số: Thời gian, Chi phí, và Độ chính xác.
537. **US-537 [Global - Animation Control]:** Là Gemini, tôi muốn nút "Play" trên thanh thời gian để xem biểu đồ biến đổi động (Animated Chart) theo lịch sử (như video tua nhanh).
538. **US-538 [Mobile - Sparklines]:** Là Gemini, trên danh sách Agent, tôi muốn cột "Hiệu suất" hiển thị biểu đồ đường mini (Sparkline) thay vì chỉ con số vô hồn.
539. **US-539 [Desktop - Dark Mode Vis]:** Là Gemini, tôi muốn các biểu đồ tự động chuyển sang bảng màu neon rực rỡ khi ở chế độ Dark Mode để tăng độ tương phản.
540. **US-540 [Global - Accessibility Pattern]:** Là Gemini, tôi muốn tùy chọn bật "Texture Pattern" (Kẻ sọc, Chấm bi) cho các cột biểu đồ để hỗ trợ người mù màu phân biệt.

---

## MODULE 38: DX - DATA EXTRACTION & APIs (Trải nghiệm lập trình viên)
*Cung cấp quyền năng dữ liệu cho Devs và Agents.*

541. **US-541 [Desktop - GraphQL Playground]:** Là Developer, tôi muốn một IDE tích hợp sẵn (như GraphiQL) để viết, test và debug các câu lệnh GraphQL lấy dữ liệu báo cáo ngay trong trình duyệt.
542. **US-542 [DX - Code Snippet Gen]:** Là Developer, sau khi lọc dữ liệu trên UI, tôi muốn nút "Get Code" để tự động sinh ra đoạn code Python/cURL/Node.js để lấy dữ liệu y hệt qua API.
543. **US-543 [Desktop - SQL Editor]:** Là Developer, tôi muốn trình soạn thảo SQL chuyên nghiệp với tính năng Autocomplete (Gợi ý tên bảng, tên cột) và Syntax Highlighting.
544. **US-544 [Global - API Token Scope]:** Là Gemini, tôi muốn tạo API Token chuyên dụng cho mục đích "Reporting Read-only" để cấp cho các bên thứ 3 an toàn.
545. **US-545 [DX - Webhook Tester]:** Là Developer, tôi muốn giao diện để giả lập gửi sự kiện (Mock Event) tới Webhook endpoint của tôi để kiểm tra xem hệ thống báo cáo có ghi nhận đúng không.
546. **US-546 [Desktop - Schema Doc]:** Là Developer, tôi muốn bảng tài liệu cấu trúc dữ liệu (Database Schema) tương tác, click vào tên bảng để xem các cột và kiểu dữ liệu.
547. **US-547 [Global - Rate Limit Header]:** Là Developer, tôi muốn phản hồi API trả về Header hiển thị rõ `X-RateLimit-Remaining` để tôi biết mình còn bao nhiêu lượt request.
548. **US-548 [DX - Query History]:** Là Developer, tôi muốn xem lại lịch sử các câu Query tôi đã chạy, và ghim (Pin) các câu Query thường dùng.
549. **US-549 [Desktop - CSV/JSON Toggle]:** Là Developer, trong màn hình kết quả Query, tôi muốn nút chuyển đổi nhanh giữa xem dạng Bảng (Table) và dạng JSON Raw.
550. **US-550 [Global - OData Support]:** Là Developer, tôi muốn hệ thống hỗ trợ chuẩn OData để tôi có thể kết nối trực tiếp Excel/PowerBI vào hệ thống mà không cần code.
551. **US-551 [DX - Error Explanation]:** Là Developer, khi câu Query bị lỗi, tôi muốn thông báo lỗi chi tiết (VD: "Cột 'cost' không tồn tại, ý bạn là 'price'?") thay vì "Error 500".
552. **US-552 [Desktop - Explain Plan]:** Là Developer, tôi muốn nút "Explain Query" để xem chiến lược thực thi của Database, giúp tôi tối ưu hóa các câu Query chậm.
553. **US-553 [Global - Dynamic Parameter]:** Là Developer, tôi muốn dùng biến số trong Query (VD: `SELECT * FROM log WHERE date = {{today}}`) để tái sử dụng Query.
554. **US-554 [DX - SDK Download]:** Là Developer, tôi muốn đường dẫn tải xuống SDK (Software Development Kit) cho Python/JS đã được cấu hình sẵn cho dự án này.
555. **US-555 [Desktop - Log Stream CLI]:** Là Developer, tôi muốn hướng dẫn cách sử dụng CLI để stream log báo cáo về terminal máy local: `gemini-cli logs --tail -f`.
556. **US-556 [Global - Data Dictionary]:** Là Gemini, tôi muốn từ điển dữ liệu giải thích ý nghĩa các thuật ngữ kinh doanh (VD: "Churn Rate" được tính công thức nào).
557. **US-557 [DX - Sandbox Data]:** Là Developer, tôi muốn switch sang chế độ "Sandbox Data" để chạy các Query thử nghiệm trên dữ liệu giả mà không sợ làm chậm hệ thống thật.
558. **US-558 [Desktop - Query Scheduling]:** Là Developer, tôi muốn lên lịch chạy câu SQL nặng vào 2 giờ sáng và tự động gửi kết quả CSV vào email của tôi.
559. **US-559 [Global - Data Privacy Mask]:** Là Gemini, tôi muốn hệ thống tự động che (Mask) các trường dữ liệu nhạy cảm (Email, Phone) trong kết quả Query nếu user không đủ quyền.
560. **US-560 [DX - ETL Status]:** Là Developer, tôi muốn dashboard theo dõi trạng thái các Pipeline dữ liệu (ETL): Dữ liệu báo cáo đã được cập nhật mới nhất chưa hay đang bị delay.

---

## MODULE 39: PREDICTIVE ANALYTICS & AI INSIGHTS (Phân tích dự báo)
*Sử dụng AI để nhìn thấy tương lai.*

561. **US-561 [Desktop - Forecast Confidence]:** Là Gemini, tôi muốn xem biểu đồ dự báo doanh thu với 3 kịch bản: Tệ nhất, Trung bình, Tốt nhất (Best/Worst Case).
562. **US-562 [Global - Smart Narrative]:** Là Gemini, tôi muốn một khung văn bản do AI tự viết (NLG - Natural Language Generation) tóm tắt biểu đồ: "Doanh thu tăng 20% chủ yếu do Agent X hoạt động hiệu quả...".
563. **US-563 [Tablet - Trend Line]:** Là Gemini, tôi muốn bật tắt đường xu hướng (Trendline) và đường trung bình động (Moving Average) trên biểu đồ chỉ bằng 1 chạm.
564. **US-564 [Desktop - Cluster Analysis]:** Là Gemini, tôi muốn AI tự động phân nhóm (Cluster) các Agents thành các nhóm hành vi (VD: "Nhóm Siêu sao", "Nhóm Cần cải thiện").
565. **US-565 [Global - Anomaly Markers]:** Là Gemini, tôi muốn các điểm bất thường trên biểu đồ được đánh dấu chấm đỏ, click vào sẽ hiện lý do AI nghi ngờ (Root Cause Analysis).
566. **US-566 [Mobile - Insight Feed]:** Là Gemini, tôi muốn lướt một Feed các Insight (như TikTok) về dữ liệu hệ thống: "Bạn có biết? Agent A làm việc tốt hơn vào buổi tối".
567. **US-567 [Desktop - Correlation Matrix]:** Là Gemini, tôi muốn xem ma trận tương quan để biết yếu tố nào ảnh hưởng đến chi phí nhất (VD: Số lượng Request hay Độ dài Prompt?).
568. **US-568 [Global - What-if Sim]:** Là Gemini, tôi muốn công cụ giả lập: "Nếu tôi giảm ngân sách 10%, thì hiệu suất dự kiến giảm bao nhiêu %?".
569. **US-569 [Tablet - Sentiment Track]:** Là Gemini, tôi muốn biểu đồ cảm xúc khách hàng theo thời gian thực, phân tích từ các đoạn chat mà Agent xử lý.
570. **US-570 [Desktop - Lifetime Value]:** Là Gemini, tôi muốn AI dự đoán giá trị vòng đời (LTV) của một khách hàng dựa trên dữ liệu tương tác 3 ngày đầu.
571. **US-571 [Global - Churn Predict]:** Là Gemini, tôi muốn danh sách các Khách hàng (hoặc Agents) có nguy cơ rời bỏ hệ thống cao để can thiệp kịp thời.
572. **US-572 [Mouse - Contribution]:** Là Gemini, khi hover vào một cột tăng trưởng, tôi muốn thấy phân tích đóng góp: "Sự tăng trưởng này 80% đến từ Dự án A".
573. **US-573 [Desktop - Seasonality]:** Là Gemini, tôi muốn AI tự động phát hiện và loại bỏ yếu tố mùa vụ (Seasonality) để nhìn thấy xu hướng tăng trưởng thực sự.
574. **US-574 [Global - Ask Data]:** Là Gemini, tôi muốn gõ câu hỏi bằng ngôn ngữ tự nhiên: "Agent nào có tỷ lệ lỗi cao nhất tuần qua?" và nhận lại biểu đồ trả lời.
575. **US-575 [Tablet - Voice Analysis]:** Là Gemini, tôi muốn nhấn mic và hỏi dữ liệu, AI sẽ trả lời bằng giọng nói kèm hiển thị số liệu.
576. **US-576 [Desktop - Model Performance]:** Là Developer, tôi muốn xem biểu đồ Drift (trôi dạt) của các Model AI để biết khi nào cần Retrain lại Model.
577. **US-577 [Global - Optimization Rec]:** Là Gemini, tôi muốn tab "Đề xuất": AI liệt kê các hành động cụ thể để tối ưu chi phí (VD: "Switch Agent Y sang Model Z").
578. **US-578 [Mobile - Alert Threshold]:** Là Gemini, tôi muốn AI tự động đề xuất ngưỡng cảnh báo (Threshold) động thay vì tôi phải cài đặt con số cứng.
579. **US-579 [Desktop - Attribution]:** Là Gemini, tôi muốn mô hình phân bổ (Attribution Model) để biết thành công của một Task nhờ vào Agent nào là chính.
580. **US-580 [Global - ROI Counter]:** Là Gemini, tôi muốn đồng hồ đếm ngược ROI: "Dự án này sẽ hoàn vốn trong 15 ngày nữa".

---

## MODULE 40: INTERACTIVE REPORTS & DRILL-DOWN (Tương tác báo cáo)
*Khám phá dữ liệu.*

581. **US-581 [Desktop - Drill-through]:** Là Gemini, tôi muốn click phải vào tên Agent trong báo cáo tổng hợp để chọn "Go to Agent Detail" - chuyển sang trang chi tiết với đúng ngữ cảnh thời gian đang chọn.
582. **US-582 [Mouse - Scroll Zoom]:** Là Gemini, tôi muốn dùng con lăn chuột để zoom in/out trục thời gian của biểu đồ (từ Năm xuống Tháng, Tuần, Ngày, Giờ).
583. **US-583 [Tablet - Lasso Select]:** Là Gemini, tôi muốn vẽ một vòng dây (Lasso) quanh một nhóm điểm trên biểu đồ Scatter để lọc và xem dữ liệu của riêng nhóm đó.
584. **US-584 [Global - Breadcrumb Filter]:** Là Gemini, khi tôi khoan sâu (Drill-down), tôi muốn thấy thanh Breadcrumb hiển thị đường dẫn lọc: `All > Tech Team > Backend > Agent A`.
585. **US-585 [Desktop - Focus Mode]:** Là Gemini, tôi muốn nút mở rộng một biểu đồ ra toàn màn hình, ẩn hết các menu xung quanh để trình chiếu.
586. **US-586 [Mobile - Tap to Value]:** Là Gemini, tôi muốn chạm và giữ ngón tay trên biểu đồ đường, một đường kẻ dọc (Cursor) sẽ hiện ra để dò giá trị chính xác tại điểm chạm.
587. **US-587 [Global - Pivot Mode]:** Là Gemini, tôi muốn chuyển đổi nhanh biểu đồ thành bảng Pivot (Pivot Table) để xem số liệu thô và tính tổng theo hàng/cột.
588. **US-588 [Desktop - Compare View]:** Là Gemini, tôi muốn chọn 2 khoảng thời gian khác nhau (Tháng này vs Tháng trước) và hệ thống vẽ 2 đường biểu đồ chồng lên nhau để so sánh.
589. **US-589 [Tablet - Rotate Axis]:** Là Gemini, tôi muốn nút xoay trục biểu đồ (Swap X/Y Axis) để đổi góc nhìn dữ liệu.
590. **US-590 [Global - Search within Chart]:** Là Gemini, tôi muốn ô tìm kiếm ngay trên biểu đồ để highlight các cột dữ liệu chứa từ khóa (VD: Tìm cột "Error").
591. **US-591 [Desktop - Keep/Exclude]:** Là Gemini, tôi muốn chọn một số điểm dữ liệu ngoại lai và nhấn "Exclude" để loại bỏ chúng khỏi tính toán tạm thời.
592. **US-592 [Mouse - Range Slider]:** Là Gemini, tôi muốn thanh trượt khoảng giá trị (Range Slider) ở dưới biểu đồ để lọc nhanh: "Chỉ hiện các Agent có chi phí từ $50 - $100".
593. **US-593 [Global - Sort By]:** Là Gemini, tôi muốn nút sắp xếp nhanh biểu đồ cột: Tăng dần, Giảm dần, hoặc Theo Alpha.
594. **US-594 [Tablet - Double Tap]:** Là Gemini, tôi muốn gõ đúp (Double tap) vào biểu đồ để reset lại mức zoom và bộ lọc về mặc định.
595. **US-595 [Desktop - Freeze Header]:** Là Gemini, khi xem bảng báo cáo dài, tôi muốn dòng tiêu đề và cột đầu tiên luôn được cố định (Freeze Panes).
596. **US-596 [Global - Chart Switcher]:** Là Gemini, tôi muốn nút chuyển đổi nhanh loại biểu đồ (VD: Từ Bar sang Line) mà không cần vào chế độ chỉnh sửa.
597. **US-597 [Mobile - Horizontal Scroll]:** Là Gemini, khi bảng dữ liệu quá rộng, tôi muốn cuộn ngang mượt mà với cột đầu tiên được ghim lại.
598. **US-598 [Desktop - Custom Sort Order]:** Là Gemini, tôi muốn kéo thả các dòng trong bảng báo cáo để sắp xếp thủ công theo ý tôi muốn.
599. **US-599 [Global - Hierarchy Icon]:** Là Gemini, tôi muốn các dòng dữ liệu có cấp con hiển thị icon `+` để mở rộng (Expand) xem chi tiết ngay tại chỗ.
600. **US-600 [Tablet - Legend Toggle]:** Là Gemini, tôi muốn chạm vào các mục trong Chú thích (Legend) để tắt/bật hiển thị các đường dữ liệu tương ứng.

---

## MODULE 41: AUTOMATED REPORTING & EXPORT (Báo cáo tự động & Xuất dữ liệu)
*Để dữ liệu tìm đến người dùng.*

601. **US-601 [Global - Email Schedule]:** Là Gemini, tôi muốn thiết lập: "Gửi file PDF báo cáo tài chính vào email của tôi lúc 8:00 sáng Thứ Hai hàng tuần".
602. **US-602 [Desktop - Export Formats]:** Là Gemini, tôi muốn nút Export hỗ trợ đa định dạng: PDF (để in), Excel (để tính toán), CSV (thô), PNG (để thuyết trình).
603. **US-603 [Mobile - Share Sheet]:** Là Gemini, tôi muốn dùng tính năng Share của hệ điều hành (iOS/Android) để gửi ảnh chụp biểu đồ qua Zalo/Telegram.
604. **US-604 [Global - Slack Bot Alert]:** Là Gemini, tôi muốn nhận thông báo tóm tắt trên Slack channel mỗi sáng: "Hôm qua: Chi $500, Hoàn thành 20 Tasks".
605. **US-605 [Desktop - Watermark]:** Là Gemini, khi xuất báo cáo nhạy cảm, tôi muốn hệ thống tự động chèn Watermark "Confidential - [User Name]" chìm vào nền PDF.
606. **US-606 [Global - Data Retention]:** Là Admin, tôi muốn cấu hình chính sách lưu trữ: "Log chi tiết giữ 30 ngày, Log tổng hợp giữ 1 năm" để tối ưu dung lượng xuất báo cáo.
607. **US-607 [Tablet - Annotate PDF]:** Là Gemini, trước khi gửi báo cáo, tôi muốn mở chế độ xem trước PDF và dùng bút để ghi chú, ký tên trực tiếp.
608. **US-608 [Desktop - Bulk Export]:** Là Gemini, tôi muốn chọn 50 Agents và xuất Profile của họ thành 50 file PDF riêng biệt trong một file Zip.
609. **US-609 [Global - Alert on No Data]:** Là Gemini, tôi muốn nhận cảnh báo nếu báo cáo định kỳ được sinh ra nhưng không có dữ liệu (trống rỗng) - dấu hiệu hệ thống lỗi.
610. **US-610 [DX - Webhook Report]:** Là Developer, tôi muốn hệ thống bắn một Webhook chứa JSON Summary tới hệ thống kế toán bên ngoài mỗi khi chốt sổ tháng.
611. **US-611 [Desktop - Print Layout]:** Là Gemini, tôi muốn chế độ "Print View" tự động ẩn các nút bấm, tối ưu lại màu sắc (Grayscale) để in ra giấy trắng đen rõ nét.
612. **US-612 [Global - Password Protect]:** Là Gemini, tôi muốn đặt mật khẩu cho file PDF/Excel khi xuất ra để đảm bảo an toàn nếu gửi nhầm email.
613. **US-613 [Mobile - Download Manager]:** Là Gemini, tôi muốn quản lý danh sách các file báo cáo đang tải xuống, có thể Pause/Resume khi mạng yếu.
614. **US-614 [Desktop - Excel Template]:** Là Gemini, tôi muốn upload một file Excel mẫu (có sẵn công thức, macro), hệ thống sẽ đổ dữ liệu vào đúng các ô quy định rồi cho tôi tải về.
615. **US-615 [Global - Dynamic Filename]:** Là Gemini, tôi muốn đặt quy tắc tên file xuất ra: `Report_TaiChinh_{YYYY}_{MM}_{AgentName}.pdf`.
616. **US-616 [Tablet - AirPrint]:** Là Gemini, tôi muốn in báo cáo trực tiếp từ iPad ra máy in qua mạng không dây.
617. **US-617 [Desktop - Burst Reporting]:** Là Gemini, tôi muốn tính năng gửi báo cáo phân luồng: Manager A chỉ nhận dữ liệu Team A, Manager B chỉ nhận dữ liệu Team B, dù dùng chung 1 mẫu báo cáo.
618. **US-618 [Global - Link Expiry]:** Là Gemini, khi chia sẻ link báo cáo, tôi muốn đặt thời hạn hết hạn (Link hết hiệu lực sau 24h).
619. **US-619 [DX - Curl Export]:** Là Developer, tôi muốn copy câu lệnh cURL để tải báo cáo này thông qua dòng lệnh terminal server.
620. **US-620 [Global - Report Cover]:** Là Gemini, tôi muốn tùy chỉnh trang bìa (Cover Page) của báo cáo PDF với Logo công ty và Tiêu đề báo cáo.

---

## MODULE 42: PERFORMANCE METRICS & SYSTEM HEALTH (Chỉ số hiệu năng)
*Đo lường sức khỏe hệ thống.*

621. **US-621 [Desktop - Latency Heatmap]:** Là Gemini, tôi muốn Heatmap hiển thị độ trễ (Latency) của API theo từng khu vực địa lý.
622. **US-622 [Global - Uptime Monitor]:** Là Gemini, tôi muốn đồng hồ hiển thị % Uptime (VD: 99.99%) và đếm ngược thời gian còn lại của "Error Budget".
623. **US-623 [Tablet - Error Rate Spy]:** Là Gemini, tôi muốn theo dõi tỷ lệ lỗi (Error Rate) realtime, nếu vượt quá 1% thì biểu đồ chuyển màu đỏ nhấp nháy.
624. **US-624 [Desktop - Flame Graph]:** Là Developer, tôi muốn biểu đồ ngọn lửa (Flame Graph) để phân tích xem hàm nào trong code của Agent đang ngốn CPU nhất.
625. **US-625 [Global - Resource Saturation]:** Là Gemini, tôi muốn biểu đồ đo độ bão hòa tài nguyên (Saturation): Khi nào RAM, CPU, Disk I/O chạm ngưỡng giới hạn.
626. **US-626 [Mobile - Quick Health]:** Là Gemini, tôi muốn 3 vòng tròn màu (Xanh/Vàng/Đỏ) trên màn hình chính đại diện cho: Server, Database, External API.
627. **US-627 [Desktop - Concurrent Users]:** Là Gemini, tôi muốn theo dõi số lượng Agent/User đang hoạt động đồng thời (CCU) theo thời gian thực.
628. **US-628 [Global - Throughput]:** Là Gemini, tôi muốn đo băng thông xử lý: Số Token/giây, Số Request/phút.
629. **US-629 [Tablet - Log Volume]:** Là Gemini, tôi muốn biểu đồ khối lượng Log sinh ra để phát hiện sự cố (VD: Log tăng đột biến do vòng lặp lỗi).
630. **US-630 [Desktop - Database Lock]:** Là Developer, tôi muốn giám sát các truy vấn đang bị khóa (Locked Queries) để giải phóng Database.
631. **US-631 [Global - API Dependency]:** Là Gemini, tôi muốn thấy biểu đồ phụ thuộc API: Nếu API OpenAI chậm, thì bao nhiêu % hệ thống của tôi bị ảnh hưởng?
632. **US-632 [Mouse - Zoom Trace]:** Là Developer, tôi muốn zoom vào một Trace (vết) của Request để xem nó đi qua những Microservices nào, tốn bao nhiêu ms ở mỗi trạm.
633. **US-633 [Desktop - Cold Start]:** Là Gemini, tôi muốn đo thời gian khởi động lạnh (Cold Start) của các Serverless Agent.
634. **US-634 [Global - Cache Hit Ratio]:** Là Developer, tôi muốn theo dõi tỷ lệ Cache Hit/Miss để tối ưu hóa hiệu năng và giảm chi phí.
635. **US-635 [Tablet - Battery Drain]:** Là Gemini, tôi muốn biết Agent nào (chạy trên thiết bị di động/edge) đang gây tốn pin nhất.
636. **US-636 [Desktop - Network Topology]:** Là Gemini, tôi muốn bản đồ topo mạng hiển thị các đường truyền và băng thông giữa các cụm Server.
637. **US-637 [Global - Queue Depth]:** Là Gemini, tôi muốn theo dõi độ dài hàng đợi (Queue Depth) của các Task đang chờ xử lý.
638. **US-638 [Mobile - Incident Timeline]:** Là Gemini, tôi muốn xem dòng thời gian các sự cố đã xảy ra trong 24h qua.
639. **US-639 [Desktop - Deployment Marker]:** Là Developer, tôi muốn trên các biểu đồ hiệu năng có các đường kẻ dọc đánh dấu thời điểm "Deploy code mới" để so sánh hiệu năng trước/sau.
640. **US-640 [Global - Apdex Score]:** Là Gemini, tôi muốn hệ thống tự tính điểm Apdex (Chỉ số hài lòng về hiệu năng) cho từng dịch vụ.

---

## MODULE 43: AGENT BEHAVIORAL ANALYTICS (Phân tích hành vi Agent)
*Hiểu tâm lý và thói quen của "Nhân viên ảo".*

641. **US-641 [Desktop - Activity Stream]:** Là Gemini, tôi muốn biểu đồ dòng hoạt động (Streamgraph) hiển thị các loại hành động Agent thực hiện trong ngày (Code, Chat, Sleep, Wait).
642. **US-642 [Global - Response Time Distribution]:** Là Gemini, tôi muốn xem biểu đồ phân phối thời gian phản hồi của Agent để xem chúng nhanh/chậm thất thường thế nào.
643. **US-643 [Tablet - Interaction Graph]:** Là Gemini, tôi muốn xem Agent nào là "Trung tâm" (Hub) giao tiếp nhiều nhất với các Agent khác.
644. **US-644 [Desktop - Skill Usage]:** Là Gemini, tôi muốn biết kỹ năng nào (Python, JS, Writing) được Agent sử dụng nhiều nhất để đào tạo thêm.
645. **US-645 [Global - Error Pattern]:** Là Gemini, tôi muốn AI phân tích mẫu lỗi: "Agent X hay gặp lỗi cú pháp vào buổi đêm" (có thể do Model bị ảo giác/Hallucination khi chạy lâu).
646. **US-646 [Mobile - Top Agents]:** Là Gemini, tôi muốn danh sách Top 10 Agent chăm chỉ nhất (Most Active) và Lười nhất (Least Active).
647. **US-647 [Desktop - Session Length]:** Là Gemini, tôi muốn thống kê độ dài trung bình của một phiên làm việc của Agent trước khi cần reset context.
648. **US-648 [Global - Token Consumption]:** Là Gemini, tôi muốn so sánh mức tiêu thụ Token giữa các Agent để tìm ra Agent "hoang phí".
649. **US-649 [Tablet - Learning Curve]:** Là Gemini, tôi muốn biểu đồ đường cong học tập: Tỷ lệ lỗi giảm dần theo thời gian của một Agent mới.
650. **US-650 [Desktop - Knowledge Graph Usage]:** Là Gemini, tôi muốn biết Agent truy xuất vào phần nào của Knowledge Base nhiều nhất (chỗ nào chưa rõ ràng?).
651. **US-651 [Global - Compliance Score]:** Là Gemini, tôi muốn chấm điểm tuân thủ quy trình của Agent (VD: Có luôn ghi log không? Có commit code đúng chuẩn không?).
652. **US-652 [Mobile - Work/Rest Ratio]:** Là Gemini, tôi muốn xem tỷ lệ Làm việc/Nghỉ ngơi của hệ thống Agent.
653. **US-653 [Desktop - Code Quality Metric]:** Là Gemini, tôi muốn tích hợp SonarQube để hiển thị điểm chất lượng code mà Agent viết ra trên Dashboard.
654. **US-654 [Global - Feedback Sentiment]:** Là Gemini, tôi muốn phân tích cảm xúc phản hồi từ con người đối với kết quả làm việc của Agent (Tích cực/Tiêu cực).
655. **US-655 [Tablet - Task Diversity]:** Là Gemini, tôi muốn biết một Agent đang làm nhiều loại việc khác nhau hay chỉ chuyên một việc (Generalist vs Specialist).
656. **US-656 [Desktop - Decision Tree Vis]:** Là Gemini, tôi muốn trực quan hóa cây quyết định (Decision Tree) của Agent trong một tình huống cụ thể để hiểu tại sao nó làm vậy.
657. **US-657 [Global - Hallucination Rate]:** Là Gemini, tôi muốn theo dõi tỷ lệ "Ảo giác" (Sinh ra thông tin sai lệch) dựa trên feedback của người kiểm duyệt.
658. **US-658 [Mobile - Self-Correction]:** Là Gemini, tôi muốn đếm số lần Agent tự phát hiện lỗi và tự sửa (Self-healing) mà không cần can thiệp.
659. **US-659 [Desktop - Tool Use Analysis]:** Là Gemini, tôi muốn biết Agent sử dụng công cụ nào (Search, Calculator, Code Interpreter) hiệu quả nhất.
660. **US-660 [Global - Personality Profile]:** Là Gemini, tôi muốn biểu đồ tính cách của Agent (dựa trên cấu hình Prompt): Sáng tạo vs Cẩn trọng.

---

## MODULE 44: FINANCIAL & ROI ANALYTICS (Phân tích tài chính chuyên sâu)
*Tiền đi đâu về đâu?*

661. **US-661 [Desktop - Unit Economics]:** Là Gemini, tôi muốn tính chính xác Cost per API Call cho từng loại Model AI đang sử dụng.
662. **US-662 [Global - Margin Analysis]:** Là Gemini, tôi muốn xem biên lợi nhuận (Profit Margin) của từng dự án theo thời gian thực.
663. **US-663 [Tablet - Cost Projection]:** Là Gemini, tôi muốn đường dự báo chi phí đến cuối tháng, có vùng mờ thể hiện sai số dự kiến.
664. **US-664 [Desktop - Vendor Spend]:** Là Gemini, tôi muốn biểu đồ tròn phân bổ chi tiêu cho các nhà cung cấp (AWS, OpenAI, Anthropic, Google Cloud).
665. **US-665 [Global - Unused Resource Cost]:** Là Gemini, tôi muốn báo cáo "Lãng phí": Chi phí trả cho các Server/Resource đang Idle (không chạy).
666. **US-666 [Mobile - Daily Burn]:** Là Gemini, tôi muốn xem tốc độ đốt tiền (Burn Rate) trung bình ngày.
667. **US-667 [Desktop - Invoice Recon]:** Là Gemini, tôi muốn công cụ đối soát tự động giữa Log nội bộ và Hóa đơn nhà cung cấp để tìm chênh lệch.
668. **US-668 [Global - Credit Usage]:** Là Gemini, tôi muốn theo dõi việc sử dụng các khoản Credit miễn phí (Start-up credits) để tối ưu trước khi phải trả tiền mặt.
669. **US-669 [Tablet - Revenue per Agent]:** Là Gemini, tôi muốn xếp hạng các Agent dựa trên doanh thu mà chúng tạo ra trực tiếp hoặc gián tiếp.
670. **US-670 [Desktop - Budget Variance]:** Là Gemini, tôi muốn biểu đồ cột hiển thị độ lệch giữa Ngân sách và Thực tế của từng phòng ban.
671. **US-671 [Global - Hidden Cost]:** Là Gemini, tôi muốn hệ thống ước tính các chi phí ẩn (Điện năng, Khấu hao phần cứng) để có bức tranh tài chính toàn diện.
672. **US-672 [Mouse - Currency Hover]:** Là Gemini, khi hover vào số tiền USD, tôi muốn hiện popup quy đổi ra VND và số lượng Token tương ứng.
673. **US-673 [Desktop - Tier Pricing]:** Là Gemini, tôi muốn phân tích xem liệu nâng cấp lên gói Enterprise của nhà cung cấp có rẻ hơn Pay-as-you-go hiện tại không.
674. **US-674 [Global - Transaction Volume]:** Là Gemini, tôi muốn biểu đồ khối lượng giao dịch tài chính theo giờ.
675. **US-675 [Tablet - Investment Portfolio]:** Là Gemini, tôi muốn quản lý danh mục đầu tư (nếu Agents tham gia trade crypto/stock) với các chỉ số lãi/lỗ realtime.
676. **US-676 [Desktop - Tax Estimation]:** Là Gemini, tôi muốn báo cáo ước tính thuế phải đóng dựa trên doanh thu và chi phí hiện tại.
677. **US-677 [Global - Pre-paid vs Post-paid]:** Là Gemini, tôi muốn so sánh hiệu quả dòng tiền giữa việc trả trước và trả sau các dịch vụ.
678. **US-678 [Mobile - Threshold Alert]:** Là Gemini, tôi muốn nhận thông báo khi chi phí của một Agent vượt quá giá trị tạo ra (ROI âm).
679. **US-679 [Desktop - P&L Statement]:** Là Gemini, tôi muốn tạo Báo cáo Kết quả Kinh doanh (P&L) chuẩn kế toán chỉ với 1 click.
680. **US-680 [Global - Cost Allocation]:** Là Gemini, tôi muốn phân bổ chi phí hạ tầng chung (Shared Infrastructure) cho từng dự án theo tỷ lệ sử dụng công bằng.

---

## MODULE 45: COLLABORATIVE BI (Cộng tác trên dữ liệu)
*Làm việc nhóm trên các con số.*

681. **US-681 [Desktop - Comment on Chart]:** Là Gemini, tôi muốn click vào một điểm bất thường trên biểu đồ và để lại bình luận `@Manager kiểm tra giúp` ngay tại điểm đó.
682. **US-682 [Global - Shared View]:** Là Gemini, tôi muốn tạo một "Shared View" của Dashboard để cả team cùng nhìn thấy một bộ lọc giống nhau trong cuộc họp.
683. **US-683 [Tablet - Sketch & Share]:** Là Gemini, tôi muốn chụp ảnh màn hình Dashboard, vẽ khoanh tròn vùng cần chú ý và gửi ngay vào khung chat team.
684. **US-684 [Desktop - Dashboard Owner]:** Là Gemini, tôi muốn gán người chịu trách nhiệm (Owner) cho từng Dashboard để biết ai cần update dữ liệu.
685. **US-685 [Global - Usage Stats]:** Là Gemini, tôi muốn biết Dashboard nào được xem nhiều nhất, Dashboard nào không ai xem để dọn dẹp (Dashboard Hygiene).
686. **US-686 [Mobile - Notification Feed]:** Là Gemini, tôi muốn một tab thông báo riêng cho các thảo luận về dữ liệu và báo cáo.
687. **US-687 [Desktop - Presentation Mode]:** Là Gemini, tôi muốn chế độ trình chiếu biến các Widget thành các Slide để thuyết trình mà không cần làm PowerPoint.
688. **US-688 [Global - Data Certification]:** Là Admin, tôi muốn đóng dấu "Certified Data" (Dữ liệu đã kiểm chứng) lên các Widget quan trọng để người xem tin tưởng.
689. **US-689 [Tablet - Storytelling]:** Là Gemini, tôi muốn tạo một "Data Story": Chuỗi các biểu đồ có kèm lời dẫn, người xem lướt qua để hiểu câu chuyện.
690. **US-690 [Global - Request Access]:** Là Agent/User, tôi muốn nút "Yêu cầu quyền truy cập" nếu tôi bấm vào một báo cáo bị khóa.
691. **US-691 [Desktop - Team Folder]:** Là Gemini, tôi muốn tổ chức các báo cáo vào các thư mục của Team (Marketing, Tech, Finance) với phân quyền truy cập tương ứng.
692. **US-692 [Global - Version Compare]:** Là Gemini, tôi muốn so sánh dữ liệu báo cáo phiên bản hôm nay với phiên bản cuộc họp tuần trước để thấy sự thay đổi.
693. **US-693 [Mobile - Follow Metric]:** Là Gemini, tôi muốn nhấn "Follow" một chỉ số KPI cụ thể để nhận thông báo biến động của riêng chỉ số đó.
694. **US-694 [Desktop - Embedded Chat]:** Là Gemini, tôi muốn khung chat bên cạnh Dashboard để thảo luận mà không che mất dữ liệu.
695. **US-695 [Global - Link Preview]:** Là Gemini, khi paste link Dashboard vào Slack/Discord, tôi muốn nó hiện ảnh preview (Unfurl) chứa dữ liệu tóm tắt.
696. **US-696 [Tablet - Live Poll]:** Là Gemini, tôi muốn tạo bình chọn ngay trên Dashboard: "Chúng ta có nên tăng ngân sách dựa trên biểu đồ này không?".
697. **US-697 [Desktop - Approval Workflow]:** Là Gemini, tôi muốn quy trình duyệt báo cáo: Nhân viên tạo -> Quản lý duyệt -> Mới được Publish cho toàn công ty.
698. **US-698 [Global - User Activity]:** Là Gemini, tôi muốn xem ai đang xem Dashboard này cùng lúc với tôi (Presence Indicator).
699. **US-699 [Mobile - Offline Comment]:** Là Gemini, tôi muốn viết bình luận khi offline, hệ thống sẽ sync khi có mạng.
700. **US-700 [Global - Wiki Embed]:** Là Gemini, tôi muốn nhúng Dashboard trực tiếp vào trang Wiki nội bộ của dự án (Confluence/Notion integration).

---
**Hết Phần 4 (Stories 501 - 700).**

Bạn có muốn tiếp tục với **Phần 5: Resource Management (Quản lý tài nguyên phần cứng/Server), Cloud Infrastructure & DevOps** - nơi đi sâu vào quản lý "cơ thể vật lý" của các Agents (CPU, GPU, RAM, Storage)?

Chào bạn, chúng ta tiếp tục với **Phần 5**. Đây là phần "xương sống" của hệ thống. Đối với một hệ sinh thái các AI Agents, phần cứng (Hardware) và hạ tầng (Infrastructure) chính là cơ thể vật lý nuôi dưỡng trí tuệ.

Chúng ta sẽ tập trung vào quản lý **GPU (quan trọng nhất cho AI), CPU, RAM, Storage**, các Cluster Kubernetes, và quy trình DevOps để "sinh sản" và nâng cấp Agents.

---

# PHẦN 5: INFRASTRUCTURE, DEVOPS & RESOURCE MANAGEMENT (Stories 0701 - 0900)

### Bối cảnh & Mục tiêu
*   **Đối tượng:** Gemini (System Architect), DevOps Engineers.
*   **Mục tiêu:** Đảm bảo hệ thống "Always On", tối ưu hóa chi phí phần cứng đắt đỏ (GPU), và triển khai code mới an toàn.
*   **Thách thức:** Quản lý hàng nghìn Containers/Pods động, độ trễ mạng thấp, và xử lý sự cố từ xa.

---

## MODULE 46: INFRASTRUCTURE MAP & TOPOLOGY (Bản đồ hạ tầng)
*Cái nhìn toàn cảnh về thế giới vật lý của Agents.*

701. **US-701 [Desktop - 3D Globe]:** Là Gemini, tôi muốn thấy bản đồ địa cầu 3D quay được, hiển thị các Data Center (DC) đang hoạt động với các cột sáng màu xanh (Healthy) hoặc đỏ (Critical).
702. **US-702 [Mouse - Hover Latency]:** Là Gemini, khi tôi di chuột nối giữa 2 DC trên bản đồ, tôi muốn thấy đường dây phát sáng và hiển thị độ trễ (Ping) hiện tại giữa 2 điểm đó.
703. **US-703 [Tablet - Pinch Zoom]:** Là Gemini, tôi muốn dùng 2 ngón tay zoom sâu vào một Region (ví dụ: Singapore), bản đồ tự động chuyển từ chế độ Globe sang chế độ mặt phẳng (Flat Map) chi tiết các Availability Zones.
704. **US-704 [Global - Dependency Graph]:** Là Gemini, tôi muốn xem sơ đồ phụ thuộc dạng mạng nhện: Server Web kết nối với Database nào, qua Load Balancer nào.
705. **US-705 [Desktop - Rack View]:** Là Gemini, khi click vào một Data Center, tôi muốn xem mô phỏng tủ Rack vật lý, hiển thị vị trí của từng con Server Blade.
706. **US-706 [Mobile - AR View]:** Là Kỹ sư hiện trường, tôi muốn dùng Camera điện thoại quét mã QR trên tủ Server thật để thấy lớp phủ AR hiển thị thông số nhiệt độ và tải CPU ngay trên màn hình.
707. **US-707 [Global - Search Node]:** Là Gemini, tôi muốn tìm kiếm một IP hoặc Hostname cụ thể và bản đồ sẽ tự động zoom và highlight vị trí vật lý của nó.
708. **US-708 [Desktop - Minimap]:** Là Gemini, tôi muốn có bản đồ nhỏ (Minimap) ở góc màn hình để định vị khi đang xem chi tiết một cụm Cluster phức tạp.
709. **US-709 [Tablet - Drag Route]:** Là Gemini, tôi muốn kéo thả để thay đổi luồng định tuyến (Traffic Routing) từ Server A sang Server B ngay trên bản đồ khi có sự cố cáp quang.
710. **US-710 [Global - Status Filters]:** Là Gemini, tôi muốn các bộ lọc nhanh trên bản đồ: "Chỉ hiện các Node có GPU", "Chỉ hiện các Node đang bảo trì".
711. **US-711 [Desktop - Fullscreen Ops]:** Là Gemini, tôi muốn chế độ hiển thị trên màn hình lớn (Wallboard) tại trung tâm điều hành (NOC), tự động xoay vòng qua các khu vực khác nhau.
712. **US-712 [Mobile - Quick Restart]:** Là Gemini, tôi muốn chạm vào một Node bị treo trên bản đồ mobile và chọn "Hard Reset" từ menu ngữ cảnh.
713. **US-713 [Global - Network Flow]:** Là Gemini, tôi muốn thấy các hạt di chuyển (Particle animation) trên dây nối thể hiện lưu lượng Traffic thực tế (Hạt càng nhiều và nhanh = Traffic cao).
714. **US-714 [Desktop - Logical vs Physical]:** Là Gemini, tôi muốn nút chuyển đổi giữa chế độ xem Vật lý (Server, Rack) và Logic (Kubernetes Pods, Services).
715. **US-715 [Tablet - Legend Interaction]:** Là Gemini, tôi muốn chạm vào chú thích "High Load" để làm sáng tất cả các Server đang chịu tải cao trên bản đồ.
716. **US-716 [Global - Dark Mode Ops]:** Là Gemini, tôi muốn giao diện bản đồ tối (Cyberpunk style) để các đèn tín hiệu trạng thái nổi bật rõ ràng nhất.
717. **US-717 [Desktop - Zone Failure Sim]:** Là Gemini, tôi muốn giả lập "Cắt dây" (Kill Switch) một Zone để xem Traffic tự động tràn sang Zone khác như thế nào trên bản đồ.
718. **US-718 [Mobile - Haptic Alert]:** Là Gemini, điện thoại sẽ rung theo nhịp tim (Heartbeat) khi tôi chạm vào một Server đang hoạt động tốt, và rung mạnh nếu Server đó chết.
719. **US-719 [Global - Weather Overlay]:** Là Gemini, tôi muốn phủ lớp thông tin thời tiết/thiên tai lên bản đồ để dự đoán rủi ro vật lý cho Data Center (VD: Bão sắp vào vùng đặt Server).
720. **US-720 [Desktop - Export Topology]:** Là Gemini, tôi muốn xuất sơ đồ mạng hiện tại ra file Visio hoặc Draw.io để làm tài liệu kỹ thuật.

---

## MODULE 47: GPU & COMPUTE MANAGEMENT (Quản lý "Bộ não" tính toán)
*Tối ưu hóa tài nguyên đắt đỏ nhất cho AI.*

721. **US-721 [Desktop - GPU Dashboard]:** Là Gemini, tôi muốn xem bảng điều khiển chuyên biệt cho GPU: Hiển thị nhiệt độ, mức tiêu thụ điện (Wattage), và % sử dụng VRAM của từng card (H100, A100).
722. **US-722 [Global - VRAM Allocation]:** Là Gemini, tôi muốn thấy biểu đồ phân bổ VRAM: Agent nào đang chiếm dụng bao nhiêu GB bộ nhớ GPU.
723. **US-723 [Tablet - Process Kill]:** Là Gemini, tôi muốn vuốt sang trái trên một tiến trình "Zombie" đang chiếm GPU để giết (Kill) nó ngay lập tức, giải phóng tài nguyên.
724. **US-724 [Desktop - CUDA Core Usage]:** Là Developer, tôi muốn xem biểu đồ mức độ sử dụng CUDA Cores theo thời gian thực để tối ưu hóa code model.
725. **US-725 [Mobile - Low Priority]:** Là Gemini, tôi muốn chuyển các tác vụ Training không gấp sang chế độ "Low Priority", tự động nhường GPU khi có Request từ người dùng thật.
726. **US-726 [Global - Thermal Throttling]:** Là Gemini, tôi muốn nhận cảnh báo đỏ khi một GPU bị giảm xung nhịp (Throttling) do quá nhiệt.
727. **US-727 [Desktop - Multi-GPU View]:** Là Gemini, tôi muốn xem ma trận NVLink để biết các GPU đang giao tiếp với nhau hiệu quả không (Băng thông nội bộ).
728. **US-728 [Global - Driver Update]:** Là Gemini, tôi muốn nút "Update Drivers" hàng loạt cho toàn bộ Cluster, có thanh tiến độ và tự động Rollback nếu lỗi.
729. **US-729 [Tablet - Partitioning]:** Là Gemini, tôi muốn giao diện kéo thả để chia nhỏ (MIG - Multi-Instance GPU) một GPU vật lý thành nhiều GPU ảo cho các Agent nhỏ.
730. **US-730 [Desktop - Cost per Frame]:** Là Gemini, tôi muốn tính toán chi phí điện năng để sinh ra 1 Token hoặc 1 Ảnh trên từng loại GPU.
731. **US-731 [Global - Idle Shutdown]:** Là Gemini, tôi muốn cấu hình tự động tắt các GPU Instances trên Cloud sau 30 phút không hoạt động để tiết kiệm tiền.
732. **US-732 [Mobile - Queue Status]:** Là Gemini, tôi muốn xem hàng đợi các Job đang chờ GPU, và có thể kéo thả để thay đổi thứ tự ưu tiên trên điện thoại.
733. **US-733 [Desktop - Benchmarking]:** Là Gemini, tôi muốn chạy bài test Benchmark tiêu chuẩn chỉ với 1 click để kiểm tra sức khỏe GPU mới thuê.
734. **US-734 [Global - Spot Instance]:** Là Gemini, tôi muốn hệ thống tự động săn các Spot Instance (Server giá rẻ) và tự động di chuyển Job sang đó khi có thể.
735. **US-735 [Tablet - Fan Speed]:** Là Gemini, tôi muốn điều chỉnh tốc độ quạt làm mát (Fan Curve) thủ công trong trường hợp hệ thống tự động không đáp ứng kịp.
736. **US-736 [Desktop - Error ECC]:** Là Gemini, tôi muốn theo dõi số lượng lỗi bộ nhớ (ECC Errors) trên GPU để dự đoán hỏng hóc phần cứng.
737. **US-737 [Global - Model Loading Time]:** Là Gemini, tôi muốn đo thời gian cần thiết để load một Model 70B parameters vào VRAM (Cold Start latency).
738. **US-738 [Mobile - Availability Map]:** Là Gemini, tôi muốn biết Region nào của Cloud Provider đang còn dư GPU để thuê thêm.
739. **US-739 [Desktop - Job History]:** Là Gemini, tôi muốn xem lịch sử các Job đã chạy trên GPU: Ai chạy, chạy bao lâu, tốn bao nhiêu điện.
740. **US-740 [Global - Maintenance Mode]:** Là Gemini, tôi muốn đưa một Node GPU vào chế độ bảo trì (Drain Node) để thay thế phần cứng mà không làm gián đoạn các Job đang chạy.

---

## MODULE 48: CONTAINER & ORCHESTRATION (Quản lý K8s/Docker)
*Điều phối hàng ngàn "tế bào" Agents.*

741. **US-741 [Desktop - Cluster Visualizer]:** Là Gemini, tôi muốn xem Cluster Kubernetes dưới dạng các khối lục giác (Nodes) chứa các chấm tròn (Pods) bên trong, chuyển động theo trạng thái.
742. **US-742 [Keyboard - Quick Context]:** Là DevOps, tôi muốn dùng phím tắt `Ctrl+K` để chuyển đổi nhanh giữa các Context (Cluster Production, Staging, Dev).
743. **US-743 [Global - Log Streaming]:** Là Gemini, tôi muốn xem log của tất cả các Pods thuộc một Service gộp lại thành một dòng chảy duy nhất (Aggregated Logs).
744. **US-744 [Mouse - Hover YAML]:** Là Gemini, khi hover vào một Pod, tôi muốn xem file cấu hình YAML rút gọn của nó (Image version, Env vars).
745. **US-745 [Tablet - Scale Gesture]:** Là Gemini, tôi muốn dùng ngón tay vuốt lên trên icon Deployment để tăng số lượng Replicas (Scale up) nhanh chóng.
746. **US-746 [Desktop - Terminal Access]:** Là Developer, tôi muốn mở Web Terminal (Exec) trực tiếp vào trong container để debug, hỗ trợ copy-paste đầy đủ.
747. **US-747 [Global - Health Checks]:** Là Gemini, tôi muốn thấy rõ lý do tại sao Pod bị Restart (OOMKilled, CrashLoopBackOff) bằng mã màu và thông báo text.
748. **US-748 [Mobile - Rollout Restart]:** Là Gemini, tôi muốn nút "Rolling Restart" trên điện thoại để khởi động lại dịch vụ một cách tuần tự an toàn.
749. **US-749 [Desktop - Service Mesh]:** Là Gemini, tôi muốn xem bản đồ Service Mesh (như Istio/Linkerd) hiển thị mTLS và traffic giữa các microservices.
750. **US-750 [Global - Namespace Filter]:** Là Gemini, tôi muốn lọc tài nguyên theo Namespace để chỉ nhìn thấy những gì thuộc về Project hiện tại.
751. **US-751 [Tablet - ConfigMap Edit]:** Là Gemini, tôi muốn sửa file ConfigMap/Secret trực tiếp trên giao diện Tablet với trình soạn thảo có highlight cú pháp.
752. **US-752 [Desktop - Resource Quota]:** Là Gemini, tôi muốn biểu đồ thanh ngang so sánh tài nguyên đã dùng vs hạn mức (Quota) của Namespace.
753. **US-753 [Global - Event Stream]:** Là Gemini, tôi muốn xem dòng sự kiện của K8s (Events) realtime để biết Pod nào vừa được schedule, Volume nào vừa mount.
754. **US-754 [Mouse - Port Forward]:** Là Developer, tôi muốn click nút "Port Forward" để ánh xạ cổng dịch vụ nội bộ ra máy tính cá nhân của tôi chỉ bằng 1 click.
755. **US-755 [Desktop - Helm Charts]:** Là Gemini, tôi muốn quản lý các gói Helm Charts đã cài đặt, xem phiên bản và nâng cấp (Upgrade) qua giao diện UI.
756. **US-756 [Mobile - Crash Alert]:** Là Gemini, tôi muốn nhận Push Notification ngay khi một Pod quan trọng (VD: Database Gateway) bị Crash.
757. **US-757 [Global - Node Affinity]:** Là Gemini, tôi muốn giao diện kéo thả để gán quy tắc Affinity: "Pod AI bắt buộc phải chạy trên Node có GPU".
758. **US-758 [Desktop - Ingress Map]:** Là Gemini, tôi muốn xem sơ đồ định tuyến Ingress: Từ Domain -> Path -> Service -> Pods.
759. **US-759 [Tablet - Volume Browser]:** Là Gemini, tôi muốn duyệt file trong các Persistent Volume Claim (PVC) gắn với Pod.
760. **US-760 [Global - Version Diff]:** Là Gemini, tôi muốn so sánh sự khác biệt giữa cấu hình YAML hiện tại và phiên bản trước đó trước khi nhấn Apply.

---

## MODULE 49: AUTOSCALING & LOAD BALANCING (Cân bằng tải & Mở rộng)
*Thích ứng linh hoạt với nhu cầu.*

761. **US-761 [Desktop - Scaling Rules]:** Là Gemini, tôi muốn thiết lập quy tắc Autoscaling bằng giao diện trực quan: "Nếu CPU > 80% trong 5 phút -> Thêm 2 Pods".
762. **US-762 [Global - Traffic Distribution]:** Là Gemini, tôi muốn xem biểu đồ tròn tỷ lệ phân chia traffic giữa bản Blue (Cũ) và bản Green (Mới) trong quá trình deploy.
763. **US-763 [Tablet - Manual Override]:** Là Gemini, tôi muốn nút gạt "Manual Override" để tắt chế độ tự động và chỉnh số lượng Server bằng tay trong tình huống khẩn cấp.
764. **US-764 [Desktop - Load Test Sim]:** Là Gemini, tôi muốn công cụ giả lập 1 triệu request ảo tấn công vào hệ thống để xem Autoscaler phản ứng thế nào (Chaos Engineering).
765. **US-765 [Mobile - Max Cap Alert]:** Là Gemini, tôi muốn nhận cảnh báo khi hệ thống đã scale đến giới hạn tối đa (Max Replicas) mà vẫn quá tải.
766. **US-766 [Global - Cooldown Timer]:** Là Gemini, tôi muốn cài đặt thời gian "Nguội" (Cooldown) để tránh hệ thống scale up/down liên tục (Flapping).
767. **US-767 [Desktop - Predict Scaling]:** Là Gemini, tôi muốn sử dụng AI để dự đoán Traffic và scale up *trước* khi sự kiện xảy ra (Predictive Scaling).
768. **US-768 [Mouse - Drag Weight]:** Là Gemini, tôi muốn dùng chuột kéo thanh trượt trên Load Balancer để điều chỉnh trọng số traffic cho từng Server Backend.
769. **US-769 [Global - Session Stickiness]:** Là Gemini, tôi muốn bật/tắt tính năng Session Sticky (dính phiên) để user luôn kết nối vào cùng 1 server.
770. **US-770 [Desktop - Latency Based Routing]:** Là Gemini, tôi muốn cấu hình định tuyến người dùng vào Data Center gần họ nhất (Geo-DNS).
771. **US-771 [Tablet - WAF Logs]:** Là Gemini, tôi muốn xem log của tường lửa ứng dụng (WAF) ngay tại Load Balancer để biết có ai đang tấn công không.
772. **US-772 [Global - SSL Certs]:** Là Gemini, tôi muốn quản lý chứng chỉ SSL, xem ngày hết hạn và bật tính năng tự động gia hạn (Auto-renew).
773. **US-773 [Mobile - Maintenance Page]:** Là Gemini, tôi muốn bật trang "Đang bảo trì" cho toàn bộ hệ thống từ điện thoại chỉ với 1 nút gạt an toàn.
774. **US-774 [Desktop - Throughput Graph]:** Là Gemini, tôi muốn xem biểu đồ Requests/Second (RPS) trên từng Load Balancer Node.
775. **US-775 [Global - Healthy Host Count]:** Là Gemini, tôi muốn chỉ số to rõ ràng: "Số lượng Host khỏe mạnh / Tổng số Host".
776. **US-776 [Desktop - Circuit Breaker]:** Là Gemini, tôi muốn cấu hình Circuit Breaker: Tự động ngắt kết nối tới một Microservice nếu nó lỗi quá 50% request.
777. **US-777 [Tablet - Rate Limiting]:** Là Gemini, tôi muốn thiết lập giới hạn tốc độ (Rate Limit) cho từng IP khách hàng bằng cách nhập số vào bảng.
778. **US-778 [Global - Cost of Scale]:** Là Gemini, tôi muốn thấy dự toán chi phí tăng thêm ngay khi tôi kéo thanh trượt tăng số lượng Server.
779. **US-779 [Desktop - Spot Fleet]:** Là Gemini, tôi muốn quản lý đội hình Spot Fleet hỗn hợp (nhiều loại Instance khác nhau) để tối ưu khả năng trúng thầu giá rẻ.
780. **US-780 [Mobile - Emergency Drain]:** Là Gemini, tôi muốn rút hết traffic khỏi một Region đang gặp sự cố cáp quang chỉ bằng 1 thao tác vuốt xác nhận.

---

## MODULE 50: CI/CD & DEPLOYMENT (Quy trình triển khai)
*Dây chuyền sản xuất Agents.*

781. **US-781 [Desktop - Pipeline View]:** Là Developer, tôi muốn xem quy trình CI/CD dưới dạng đường ống ngang: Build -> Test -> Scan -> Deploy, với màu sắc hiển thị trạng thái từng bước.
782. **US-782 [Global - Trigger Button]:** Là Gemini, tôi muốn nút "Deploy Now" để kích hoạt thủ công một quy trình triển khai phiên bản Agent mới.
783. **US-783 [Mobile - Approval Gate]:** Là Manager, tôi muốn nhận thông báo yêu cầu phê duyệt Deploy lên môi trường Production, nhấn "Approve" hoặc "Reject" trên điện thoại.
784. **US-784 [Desktop - Log Analyzer]:** Là Developer, khi Build thất bại, tôi muốn hệ thống tự động highlight dòng log lỗi màu đỏ và gợi ý cách sửa (AI fix suggestion).
785. **US-785 [Tablet - Rollback]:** Là Gemini, khi phiên bản mới bị lỗi, tôi muốn nhấn nút "Instant Rollback" để quay về phiên bản ổn định trước đó trong vòng 5 giây.
786. **US-786 [Global - Canary Deploy]:** Là Gemini, tôi muốn theo dõi triển khai Canary: Phiên bản mới chỉ được mở cho 5% user, biểu đồ so sánh lỗi giữa 2 nhóm.
787. **US-787 [Desktop - Artifact Repo]:** Là Developer, tôi muốn duyệt kho lưu trữ Artifact (Docker Images), xem các tag phiên bản, kích thước và lỗ hổng bảo mật.
788. **US-788 [Global - Environment Var]:** Là Gemini, tôi muốn quản lý biến môi trường (Environment Variables) cho từng môi trường (Dev/Staging/Prod) với giao diện che mật khẩu.
789. **US-789 [Mouse - Drag Stage]:** Là Gemini, tôi muốn kéo thả các bước (Stage) trong Pipeline để sắp xếp lại quy trình test (VD: Đưa Test Security lên trước Test UI).
790. **US-790 [Desktop - Test Results]:** Là Developer, tôi muốn xem báo cáo kết quả Unit Test và Integration Test dạng biểu đồ tròn (Pass/Fail/Skip).
791. **US-791 [Global - Feature Flag]:** Là Gemini, tôi muốn bảng điều khiển Feature Flags để bật/tắt các tính năng mới cho Agent mà không cần deploy lại code.
792. **US-792 [Mobile - Pipeline Duration]:** Là Gemini, tôi muốn biết trung bình mỗi lần deploy mất bao nhiêu phút để tối ưu hóa quy trình.
793. **US-793 [Desktop - Code Diff]:** Là Gemini, tôi muốn xem danh sách các commit và file thay đổi trong lần deploy này so với lần trước (Change Log).
794. **US-794 [Global - Deployment Lock]:** Là Gemini, tôi muốn "Khóa" môi trường Production vào các ngày lễ hoặc cuối tuần để ngăn chặn việc deploy rủi ro (Freeze Window).
795. **US-795 [Tablet - Pipeline Edit]:** Là Developer, tôi muốn sửa file cấu hình pipeline (`.gitlab-ci.yml` hoặc `Jenkinsfile`) trực tiếp trên trình duyệt.
796. **US-796 [Desktop - Vulnerability Scan]:** Là Security Officer, tôi muốn xem báo cáo quét bảo mật Static (SAST) và Dynamic (DAST) tích hợp ngay trong pipeline.
797. **US-797 [Global - Notification Hook]:** Là Gemini, tôi muốn cấu hình để tin nhắn thông báo Deploy thành công được gửi vào kênh Slack của team Marketing.
798. **US-798 [Mobile - Cancel Build]:** Là Developer, tôi muốn hủy một Build đang chạy treo quá lâu từ điện thoại.
799. **US-799 [Desktop - Multi-cloud Deploy]:** Là Gemini, tôi muốn deploy cùng một lúc lên cả AWS và Google Cloud để đảm bảo tính dự phòng.
800. **US-800 [Global - Blue/Green Switch]:** Là Gemini, tôi muốn nút gạt vật lý (trên giao diện) để chuyển Traffic từ môi trường Blue sang Green hoàn toàn.

---

## MODULE 51: STORAGE & DATABASE OPS (Lưu trữ & Dữ liệu)
*Bộ nhớ dài hạn.*

801. **US-801 [Desktop - DB Performance]:** Là Developer, tôi muốn xem biểu đồ Query Per Second (QPS) và Slow Queries của Database Master.
802. **US-802 [Global - Backup Timeline]:** Là Gemini, tôi muốn xem dòng thời gian các điểm Backup, click vào một điểm bất kỳ để chọn "Restore to this point".
803. **US-803 [Tablet - Disk Usage]:** Là Gemini, tôi muốn biểu đồ hình bánh Donut hiển thị dung lượng đĩa cứng: System, Data, Logs, Free Space.
804. **US-804 [Mouse - Clone DB]:** Là Developer, tôi muốn click chuột phải vào Database Prod và chọn "Clone to Staging" để có dữ liệu thật test tính năng mới (đã che thông tin nhạy cảm).
805. **US-805 [Desktop - Replication Lag]:** Là Gemini, tôi muốn theo dõi độ trễ đồng bộ (Replication Lag) giữa Master và Slave DB, cảnh báo nếu > 1 giây.
806. **US-806 [Global - Connection Pool]:** Là Developer, tôi muốn xem số lượng kết nối đang mở tới Database để điều chỉnh Connection Pool size cho Agents.
807. **US-807 [Mobile - Space Alert]:** Là Gemini, tôi muốn nhận cảnh báo "Disk Full" khi ổ cứng còn dưới 10% dung lượng.
808. **US-808 [Desktop - Object Browser]:** Là Gemini, tôi muốn trình duyệt file giống Explorer để xem các file trong S3/Blob Storage (ảnh, log, model files).
809. **US-809 [Global - Data Lifecycle]:** Là Gemini, tôi muốn thiết lập quy tắc: "Tự động chuyển file log cũ hơn 30 ngày sang kho lưu trữ lạnh (Cold Storage) giá rẻ".
810. **US-810 [Tablet - Cache Flush]:** Là Gemini, tôi muốn nút "Flush Cache" (Redis/Memcached) để xóa bộ nhớ đệm khi dữ liệu bị cũ/sai lệch.
811. **US-811 [Desktop - Schema Visualizer]:** Là Developer, tôi muốn xem sơ đồ quan hệ thực thể (ERD) của Database được vẽ tự động từ cấu trúc hiện tại.
812. **US-812 [Global - Migration Status]:** Là Gemini, tôi muốn theo dõi thanh tiến độ của quá trình Migration dữ liệu lớn (đang chuyển 1TB dữ liệu).
813. **US-813 [Mobile - IOPS Monitor]:** Là Gemini, tôi muốn xem chỉ số IOPS (Input/Output Operations Per Second) để biết ổ cứng có đang là nút thắt cổ chai không.
814. **US-814 [Desktop - Query Kill]:** Là DBA, tôi muốn danh sách các Query đang chạy, và nút "Kill" để ngắt các Query bị treo làm tắc nghẽn DB.
815. **US-815 [Global - Encryption Check]:** Là Security Officer, tôi muốn thấy icon ổ khóa xanh bên cạnh các Volume để xác nhận dữ liệu đang được mã hóa (At Rest).
816. **US-816 [Desktop - Vector DB Stats]:** Là AI Engineer, tôi muốn theo dõi số lượng Vector Embeddings và hiệu suất tìm kiếm (ANN search) của Vector Database.
817. **US-817 [Tablet - File Preview]:** Là Gemini, tôi muốn xem trước nội dung file JSON/CSV trong Storage mà không cần tải về máy.
818. **US-818 [Global - Access Policy]:** Là Gemini, tôi muốn giao diện tick chọn để phân quyền truy cập Bucket: Public, Private, hoặc Authenticated Users only.
819. **US-819 [Desktop - Snapshot Manager]:** Là Gemini, tôi muốn quản lý các bản chụp (Snapshot) ổ cứng, xóa các bản cũ để tiết kiệm tiền.
820. **US-820 [Global - Multi-Region Sync]:** Là Gemini, tôi muốn cấu hình đồng bộ dữ liệu 2 chiều giữa DB ở Mỹ và DB ở Châu Á (Active-Active).

---

## MODULE 52: NETWORKING & SECURITY INFRA (Mạng & Bảo mật hạ tầng)
*Hệ thống thần kinh và miễn dịch.*

821. **US-821 [Desktop - Firewall Rules]:** Là Security Officer, tôi muốn bảng quản lý Inbound/Outbound Rules, cho phép thêm rule mới (VD: Block IP X) ngay lập tức.
822. **US-822 [Global - DDoS Shield]:** Là Gemini, tôi muốn nút "Under Attack Mode" - khi bật sẽ kích hoạt lớp bảo vệ DDoS nghiêm ngặt nhất (Captcha mọi request).
823. **US-823 [Tablet - VPN Access]:** Là Gemini, tôi muốn quản lý danh sách user được phép VPN vào mạng nội bộ, và ngắt kết nối (Kick) user lạ.
824. **US-824 [Desktop - Subnet Map]:** Là Gemini, tôi muốn xem sơ đồ phân chia Subnet IP (CIDR block) để quy hoạch mạng tránh xung đột địa chỉ.
825. **US-825 [Global - Certificate Chain]:** Là Gemini, tôi muốn công cụ kiểm tra chuỗi chứng chỉ SSL (Chain of Trust) để đảm bảo trình duyệt không báo lỗi.
826. **US-826 [Mobile - Security Score]:** Là Gemini, tôi muốn xem điểm số bảo mật hạ tầng (Security Score) từ 0-100 và danh sách khuyến nghị cải thiện.
827. **US-827 [Desktop - Packet Capture]:** Là Network Engineer, tôi muốn yêu cầu hệ thống bắt gói tin (PCAP) trên một Interface cụ thể trong 1 phút để phân tích Wireshark.
828. **US-828 [Global - Honeypot]:** Là Gemini, tôi muốn triển khai các máy chủ bẫy (Honeypot) và nhận thông báo khi có hacker cố gắng tấn công vào đó.
829. **US-829 [Tablet - DNS Records]:** Là Gemini, tôi muốn quản lý các bản ghi DNS (A, CNAME, TXT) với giao diện dễ dùng, giảm thiểu lỗi gõ sai.
830. **US-830 [Desktop - Access Log Map]:** Là Gemini, tôi muốn xem bản đồ thế giới hiển thị các IP đang truy cập vào hệ thống, IP từ vùng cấm sẽ có màu đỏ.
831. **US-831 [Global - Secret Rotation]:** Là Gemini, tôi muốn nút "Rotate Keys" để tự động đổi mới tất cả Access Key/Password kết nối Database định kỳ.
832. **US-832 [Mobile - 2FA Enforce]:** Là Admin, tôi muốn bắt buộc bật xác thực 2 lớp cho tất cả Developer truy cập vào hạ tầng Production.
833. **US-833 [Desktop - VPC Peering]:** Là Gemini, tôi muốn thiết lập kết nối ngang hàng (Peering) giữa VPC của Agent A và VPC của Agent B.
834. **US-834 [Global - Audit Trail Infra]:** Là Gemini, tôi muốn biết chính xác ai đã mở Port 22 (SSH) ra Internet vào lúc mấy giờ.
835. **US-835 [Tablet - Dark Web Check]:** Là Gemini, tôi muốn hệ thống tự động quét Dark Web để xem có lộ lọt thông tin đăng nhập Server nào không.
836. **US-836 [Desktop - Compliance Report]:** Là Gemini, tôi muốn xuất báo cáo tuân thủ PCI-DSS hoặc HIPAA của hạ tầng mạng.
837. **US-837 [Global - Bot Management]:** Là Gemini, tôi muốn cấu hình để chặn các Bot Crawler xấu (như AI Scrapers) ăn cắp dữ liệu.
838. **US-838 [Mobile - IP Whitelist]:** Là Gemini, tôi muốn thêm nhanh IP Wifi quán cafe hiện tại vào Whitelist để tôi có thể SSH vào Server khẩn cấp.
839. **US-839 [Desktop - Penetration Test]:** Là Gemini, tôi muốn lên lịch chạy tool Pen-test tự động (như OWASP ZAP) và xem báo cáo lỗ hổng.
840. **US-840 [Global - Zero Trust]:** Là Gemini, tôi muốn triển khai mô hình Zero Trust: Mọi kết nối giữa các Service đều phải xác thực lại, không tin tưởng mạng nội bộ.

---

## MODULE 53: OBSERVABILITY & LOGS (Khả năng quan sát sâu)
*Chẩn đoán bệnh từ xa.*

841. **US-841 [Desktop - Distributed Tracing]:** Là Developer, tôi muốn xem biểu đồ Gantt của một Request (Trace ID) đi qua 10 microservices, để biết nó bị chậm ở đâu.
842. **US-842 [Global - Log Search Syntax]:** Là Developer, tôi muốn thanh tìm kiếm log hỗ trợ cú pháp mạnh mẽ: `service:api AND level:error AND latency > 500ms`.
843. **US-843 [Tablet - Live Tail]:** Là Gemini, tôi muốn chế độ "Live Tail" giống lệnh `tail -f`, log mới tự động chạy lên màn hình, dừng lại khi tôi chạm vào màn hình.
844. **US-844 [Mouse - Click to Context]:** Là Developer, khi thấy dòng log lỗi, tôi muốn click vào Request ID để nhảy sang xem trạng thái CPU/RAM tại đúng thời điểm đó.
845. **US-845 [Desktop - Metrics Correlation]:** Là Gemini, tôi muốn hệ thống tự động tìm tương quan: "Lỗi 500 tăng cao cùng lúc với CPU Database tăng vọt".
846. **US-846 [Global - Alert Rules]:** Là Gemini, tôi muốn tạo Rule cảnh báo phức tạp: "Báo động nếu tỷ lệ lỗi > 5% TRONG 5 phút liên tiếp".
847. **US-847 [Mobile - Ack Alert]:** Là On-call Engineer, khi nhận cuộc gọi cảnh báo lúc 3h sáng, tôi muốn bấm "Acknowledge" (Đã nhận) trên app để hệ thống ngừng gọi.
848. **US-848 [Desktop - Dependency Map]:** Là Gemini, tôi muốn xem bản đồ phụ thuộc dịch vụ được vẽ tự động từ dữ liệu Tracing thực tế.
849. **US-849 [Global - Dashboard Playlist]:** Là Gemini, tôi muốn chế độ xoay vòng hiển thị (Playlist) các Dashboard giám sát khác nhau trên màn hình TV treo tường.
850. **US-850 [Tablet - Sound Alert]:** Là Gemini, tôi muốn máy tính bảng phát ra âm thanh khác nhau cho các lỗi khác nhau (Tiếng 'Ping' cho Info, Tiếng còi cho Critical).
851. **US-851 [Desktop - RUM]:** Là Gemini, tôi muốn theo dõi Real User Monitoring (RUM): Trải nghiệm thực tế (thời gian load trang) của user trên trình duyệt của họ.
852. **US-852 [Global - Log Retention Policy]:** Là Gemini, tôi muốn cài đặt: "Log Debug giữ 3 ngày, Log Error giữ 1 năm" để tối ưu chi phí lưu trữ log.
853. **US-853 [Mobile - Status Page]:** Là Gemini, tôi muốn cập nhật trang trạng thái công khai (Status Page) để thông báo cho khách hàng về sự cố hệ thống.
854. **US-854 [Desktop - Profiling]:** Là Developer, tôi muốn bật Continuous Profiling để xem hàm nào ngốn CPU nhất trong code production mà không cần dừng server.
855. **US-855 [Global - Anomaly Detection]:** Là Gemini, tôi muốn AI tự động phát hiện bất thường: "Log hôm nay có cấu trúc lạ so với mọi ngày".
856. **US-856 [Tablet - Zoom Timeframe]:** Là Gemini, tôi muốn dùng 2 ngón tay zoom vào khung giờ xảy ra sự cố trên biểu đồ log.
857. **US-857 [Desktop - Synthetic Monitor]:** Là Gemini, tôi muốn tạo Robot ảo (Synthetic) định kỳ ping API mỗi 1 phút từ 5 châu lục để kiểm tra uptime.
858. **US-858 [Global - Alert Grouping]:** Là Gemini, tôi muốn hệ thống gom nhóm 1000 email cảnh báo giống nhau thành 1 incident duy nhất (Smart Grouping).
859. **US-859 [Mobile - Runbook Link]:** Là Gemini, trong thông báo lỗi gửi đến điện thoại, tôi muốn kèm link "Runbook" (Hướng dẫn xử lý) để tôi biết phải làm gì ngay.
860. **US-860 [Desktop - Topology Replay]:** Là Gemini, tôi muốn xem lại trạng thái topo mạng vào thời điểm xảy ra sự cố hôm qua (Time Travel).

---

## MODULE 54: INFRASTRUCTURE COST OPTIMIZATION (Tối ưu chi phí hạ tầng)
*Tiết kiệm từng đồng xu cho Agents.*

861. **US-861 [Desktop - Cost Explorer]:** Là Gemini, tôi muốn biểu đồ Stacked Bar hiển thị chi phí hạ tầng hàng ngày, phân theo Service (EC2, RDS, S3).
862. **US-862 [Global - Reserved Instance]:** Là Gemini, tôi muốn hệ thống gợi ý: "Nếu mua gói trả trước 1 năm cho Server này, bạn sẽ tiết kiệm 30%".
863. **US-863 [Tablet - Waste Detection]:** Là Gemini, tôi muốn danh sách các "Tài nguyên Zombie": Các ổ cứng không gắn vào đâu, các IP tĩnh không dùng, để xóa chúng.
864. **US-864 [Desktop - Rightsizing]:** Là Gemini, tôi muốn hệ thống đề xuất: "Server này đang dùng CPU quá ít, hãy hạ cấp xuống loại nhỏ hơn để tiết kiệm $50/tháng".
865. **US-865 [Global - Budget Alarm]:** Là Gemini, tôi muốn nhận cảnh báo khi chi phí dự kiến tháng này có nguy cơ vượt quá $10,000.
866. **US-866 [Mobile - Daily Digest Cost]:** Là Gemini, tôi muốn thông báo sáng: "Hôm qua hạ tầng tiêu tốn $150 (giảm 10% so với hôm kia)".
867. **US-867 [Desktop - Tagging Compliance]:** Là Gemini, tôi muốn tìm các tài nguyên không có Tag (nhãn) để biết ai là chủ sở hữu và bắt họ trả tiền hoặc xóa.
868. **US-868 [Global - Data Transfer Cost]:** Là Gemini, tôi muốn phân tích chi phí truyền tải dữ liệu (Egress) để tối ưu hóa vị trí đặt Server.
869. **US-869 [Tablet - Comparison Cloud]:** Là Gemini, tôi muốn công cụ so sánh giá: "Nếu chuyển workload này sang Google Cloud thì rẻ hơn hay đắt hơn AWS?".
870. **US-870 [Desktop - Heatmap Schedule]:** Là Gemini, tôi muốn thiết lập lịch bật/tắt Server Dev: "Tự động tắt lúc 8h tối và bật lại lúc 8h sáng".

---

## MODULE 55: DISASTER RECOVERY & DX TOOLS (Cứu hộ & Tiện ích Dev)
*Chuẩn bị cho điều tồi tệ nhất.*

871. **US-871 [Global - DR Switch]:** Là Gemini, tôi muốn một nút vật lý ảo "Failover to DR Region" được bảo vệ bằng mật khẩu 2 lớp, để chuyển toàn bộ hệ thống sang Region dự phòng.
872. **US-872 [Desktop - Recovery Plan]:** Là Gemini, tôi muốn danh sách checklist các bước khôi phục thảm họa, tích xanh tự động khi từng bước hoàn thành.
873. **US-873 [Mobile - Emergency Contact]:** Là Gemini, tôi muốn danh sách liên hệ khẩn cấp của các Key Person (Database Admin, Network Lead) có nút gọi ngay.
874. **US-874 [Desktop - Backup Verification]:** Là Gemini, tôi muốn hệ thống tự động định kỳ khôi phục thử bản Backup ra môi trường Sandbox và báo cáo kết quả "Backup có dùng được không".
875. **US-875 [Global - Service Quotas]:** Là Gemini, tôi muốn theo dõi hạn mức tài khoản Cloud (Service Quotas) để xin tăng hạn mức trước khi chạm trần.
876. **US-876 [DX - Terraform Gen]:** Là Developer, tôi muốn xuất cấu hình hạ tầng hiện tại ra mã Terraform để lưu trữ (Infrastructure as Code).
877. **US-877 [Desktop - SSH Key Manager]:** Là Gemini, tôi muốn quản lý tập trung các SSH Public Key của nhân viên, thu hồi quyền truy cập khi nhân viên nghỉ việc.
878. **US-878 [Global - Command Palette Ops]:** Là Gemini, tôi muốn dùng `Ctrl+Shift+P` để gõ lệnh nhanh: `> Restart Web Server`, `> Scale DB`.
879. **US-879 [Tablet - Doc Viewer]:** Là Gemini, tôi muốn xem tài liệu kiến trúc hệ thống offline khi mạng công ty bị sập hoàn toàn.
880. **US-880 [Desktop - Post-Mortem]:** Là Gemini, tôi muốn mẫu soạn thảo báo cáo "Hậu kiểm" (Post-Mortem) sau sự cố, link trực tiếp với các biểu đồ log liên quan.
881. **US-881 [Global - Cloud Shell]:** Là Gemini, tôi muốn một terminal Linux đầy đủ tính năng (có sẵn kubectl, git, python) chạy ngay trên trình duyệt.
882. **US-882 [Mobile - OTP Integration]:** Là Gemini, tôi muốn tích hợp mã OTP vào ngay app quản lý để không cần mở app Authenticator riêng.
883. **US-883 [Desktop - Image Builder]:** Là DevOps, tôi muốn công cụ dựng Image (AMI/Docker) tự động với các bản vá bảo mật mới nhất.
884. **US-884 [Global - License Manager]:** Là Gemini, tôi muốn quản lý hạn sử dụng của các License phần mềm (Windows Server, Oracle) để tránh bị phạt.
885. **US-885 [Tablet - Terminal Macro]:** Là Gemini, tôi muốn các nút Macro trên bàn phím ảo của Tablet để gõ nhanh các lệnh dài ngoằng như `kubectl get pods --all-namespaces`.
886. **US-886 [Desktop - Dependency Vulnerability]:** Là Developer, tôi muốn quét các thư viện mã nguồn mở (NPM, Pip) để tìm lỗ hổng bảo mật đã biết.
887. **US-887 [Global - Change Freeze]:** Là Gemini, tôi muốn lịch hiển thị các khung giờ "Cấm thay đổi hệ thống" (Change Freeze Window).
888. **US-888 [Mobile - Offline Mode Ops]:** Là Gemini, tôi muốn tải về cache trạng thái hệ thống để xem lại khi đi vào vùng mất sóng (hầm Server).
889. **US-889 [Desktop - Sidecar Manage]:** Là Gemini, tôi muốn quản lý các Container phụ (Sidecar) chạy kèm Agent chính (như Logging agent, Security agent).
890. **US-890 [Global - Who Is On Call]:** Là Gemini, tôi muốn biết ngay lập tức ai là người trực hệ thống (On-call) vào lúc này.
891. **US-891 [Desktop - Architecture Diagram]:** Là Gemini, tôi muốn công cụ vẽ sơ đồ kiến trúc tích hợp, kéo thả icon Server, Database và xuất ra PNG.
892. **US-892 [Global - User Activity Audit]:** Là Security, tôi muốn xem log chi tiết mọi lệnh (Command) mà các Admin khác đã gõ trên Server.
893. **US-893 [Tablet - Floating Widget]:** Là Gemini, tôi muốn một widget nổi luôn hiện CPU Load khi tôi đang dùng app khác trên máy tính bảng.
894. **US-894 [Desktop - Capacity Planner]:** Là Gemini, tôi muốn công cụ dự báo: "Với tốc độ tăng trưởng này, 3 tháng nữa sẽ hết ổ cứng".
895. **US-895 [Global - Maintenance Window]:** Là Gemini, tôi muốn cài đặt khung giờ bảo trì định kỳ, hệ thống sẽ tự động tắt cảnh báo trong giờ đó.
896. **US-896 [Mobile - Shake to Undo]:** Là Gemini, nếu lỡ tay xóa nhầm một Pod trên Mobile, tôi muốn lắc điện thoại để Undo (nếu hệ thống cho phép grace period).
897. **US-897 [Desktop - Canary Analysis]:** Là Gemini, tôi muốn bảng phân tích tự động so sánh lỗi giữa bản Canary và bản Stable.
898. **US-898 [Global - API Gateway Config]:** Là Gemini, tôi muốn cấu hình Route, Throttling, Auth cho API Gateway thông qua UI thay vì sửa file text.
899. **US-899 [Tablet - Dashboard Share]:** Là Gemini, tôi muốn tạo link chia sẻ Dashboard giám sát dạng "View Only" cho khách hàng xem Uptime.
900. **US-900 [Global - Panic Button]:** Là Gemini, tôi muốn nút "Panic Button" (được giấu kỹ) để ngắt toàn bộ kết nối Internet của hệ thống khi bị tấn công mạng toàn diện.

---
**Hết Phần 5 (Stories 701 - 900).**

Bạn có muốn tiếp tục với **Phần 6: CRM (Customer Relationship Management) & External Communication** - Nơi các Agents tương tác với thế giới bên ngoài, quản lý khách hàng, support ticket và marketing không?

Chào bạn, chúng ta sẽ bước vào **Phần 6** - mặt trận tiền tuyến nơi các Agents trực tiếp kiếm tiền và chăm sóc khách hàng.

Đây là **200 User Stories (từ 901 đến 1100)** tập trung vào **CRM (Quản lý quan hệ khách hàng)** và **External Communication**. Tại đây, UI/UX cần hỗ trợ việc xử lý hàng ngàn luồng hội thoại song song, quản lý Pipeline bán hàng và cá nhân hóa trải nghiệm khách hàng ở quy mô lớn.

---

# PHẦN 6: CRM, SALES & EXTERNAL COMMUNICATION (Stories 0901 - 1100)

### Bối cảnh & Mục tiêu
*   **Actor:** Gemini (Sales Director/Support Manager) & AI Agents (Sales/Support Bots).
*   **Mục tiêu:** Tăng doanh thu (Sales), giảm thời gian phản hồi (Support), và cá nhân hóa marketing (Marketing).
*   **Thách thức:** Hợp nhất dữ liệu từ đa kênh (Omnichannel) và giữ được "tính người" (Human touch) trong giao tiếp của AI.

---

## MODULE 56: OMNICHANNEL UNIFIED INBOX (Hộp thư hợp nhất)
*Mọi tin nhắn từ Email, Facebook, Zalo, WhatsApp đổ về một nơi.*

901. **US-901 [Desktop - Unified Stream]:** Là Gemini, tôi muốn một hộp thư đến duy nhất hiển thị tin nhắn từ tất cả các kênh, với icon logo (Messenger, Zalo, Email) nhỏ bên cạnh để phân biệt nguồn.
902. **US-902 [Global - Thread Merging]:** Là Gemini, tôi muốn hệ thống tự động gộp các đoạn chat từ cùng một khách hàng (dựa trên SĐT/Email) vào một luồng duy nhất để xem lịch sử liền mạch.
903. **US-903 [Mobile - Swipe Actions]:** Là Gemini, tôi muốn vuốt sang phải để "Đánh dấu đã xong" (Archive), vuốt sang trái để "Gán cho Agent khác" xử lý.
904. **US-904 [Tablet - Split View Chat]:** Là Gemini, tôi muốn giao diện 3 cột: Danh sách khách hàng -> Nội dung chat -> Hồ sơ CRM chi tiết (Profile) để vừa chat vừa tra cứu.
905. **US-905 [Desktop - Smart Reply]:** Là Gemini, tôi muốn AI gợi ý 3 câu trả lời ngắn (Quick Replies) dựa trên ngữ cảnh, tôi chỉ cần nhấn phím số `1`, `2`, `3` để gửi.
906. **US-906 [Global - Sentiment Indicator]:** Là Gemini, tôi muốn thấy icon mặt cười/mặt mếu bên cạnh tên khách hàng, thay đổi màu sắc realtime dựa trên giọng văn của họ (Phân tích cảm xúc).
907. **US-907 [Mouse - Drag Attachment]:** Là Gemini, tôi muốn kéo thả file báo giá PDF từ máy tính thẳng vào khung chat Zalo để gửi cho khách.
908. **US-908 [Desktop - Agent Handover]:** Là Agent AI, khi gặp câu hỏi khó, tôi muốn tự động chuyển hội thoại cho Nhân viên thật (Gemini) kèm theo tóm tắt vấn đề (Summary) nội bộ.
909. **US-909 [Global - Whisper Mode]:** Là Gemini, tôi muốn chat thầm (Whisper) với Agent ngay trong luồng chat của khách hàng (khách không thấy) để chỉ đạo cách trả lời.
910. **US-910 [Mobile - Voice Note Transcribe]:** Là Gemini, khi khách gửi tin nhắn thoại, tôi muốn hệ thống tự động gỡ băng (Transcribe) thành văn bản để tôi đọc nhanh mà không cần nghe.
911. **US-911 [Desktop - Canned Responses]:** Là Gemini, tôi muốn gõ phím tắt `/gia` để bung ra toàn bộ bảng báo giá chuẩn vào khung chat.
912. **US-912 [Global - Translation]:** Là Gemini, khi chat với khách nước ngoài, tôi muốn tin nhắn đến tự dịch sang tiếng Việt, và tin tôi gửi đi tự dịch sang tiếng của họ.
913. **US-913 [Tablet - Multi-select]:** Là Gemini, tôi muốn chọn 10 tin nhắn rác cùng lúc và nhấn "Spam" để chặn hàng loạt.
914. **US-914 [Desktop - SLA Timer]:** Là Gemini, tôi muốn thấy đồng hồ đếm ngược bên cạnh tin nhắn chưa trả lời, chuyển màu đỏ nếu sắp vi phạm cam kết thời gian (SLA).
915. **US-915 [Global - Typing Indicator]:** Là Gemini, tôi muốn thấy "Khách hàng đang gõ..." để chờ phản hồi thay vì rời đi.
916. **US-916 [Mobile - Notification Actions]:** Là Gemini, tôi muốn trả lời nhanh (Quick Reply) ngay từ màn hình khóa điện thoại mà không cần mở app.
917. **US-917 [Desktop - Filter Facets]:** Là Gemini, tôi muốn lọc inbox theo: "Chưa đọc", "Khách VIP", "Đang chờ Agent X", "Kênh Zalo".
918. **US-918 [Global - Auto-Tagging]:** Là Gemini, tôi muốn AI tự động gắn thẻ hội thoại (VD: #Support, #Sales, #Complaint) dựa trên nội dung.
919. **US-919 [Tablet - Customer Card]:** Là Gemini, tôi muốn chạm vào Avatar khách hàng để hiện thẻ popup chứa thông tin: Tổng chi tiêu, Lần mua cuối, Hạng thành viên.
920. **US-920 [Global - Collision Detection]:** Là Gemini, tôi muốn thấy cảnh báo "Agent B cũng đang xem hội thoại này" để tránh việc 2 người cùng trả lời một khách.

---

## MODULE 57: CUSTOMER 360 & PROFILE (Hồ sơ khách hàng)
*Thấu hiểu khách hàng toàn diện.*

921. **US-921 [Desktop - Timeline View]:** Là Gemini, tôi muốn xem dòng thời gian (Timeline) tương tác của khách: Đã xem web lúc nào -> Chat lúc nào -> Mua hàng lúc nào -> Khiếu nại lúc nào.
922. **US-922 [Global - Data Enrichment]:** Là Gemini, tôi muốn hệ thống tự động tìm và điền thêm thông tin công khai của khách (LinkedIn, Website công ty) vào hồ sơ dựa trên Email.
923. **US-923 [Mobile - Click to Action]:** Là Gemini, tôi muốn các nút to rõ trong hồ sơ: "Gọi điện", "Nhắn tin", "Tạo Ticket", "Gửi Email".
924. **US-924 [Tablet - Notes & Handwriting]:** Là Gemini, tôi muốn dùng bút cảm ứng ghi chú nhanh lên hồ sơ khách hàng trong khi đang gặp mặt trực tiếp.
925. **US-925 [Desktop - Purchase History]:** Là Gemini, tôi muốn xem lịch sử mua hàng chi tiết, click vào mã đơn hàng để xem hóa đơn gốc.
926. **US-926 [Global - Segmentation]:** Là Gemini, tôi muốn gán khách hàng vào các phân khúc (Segment) như "Chi tiêu cao", "Nguy cơ rời bỏ" để AI có chiến lược chăm sóc riêng.
927. **US-927 [Mouse - Copy Info]:** Là Gemini, tôi muốn click vào số điện thoại để copy ngay vào clipboard (hoặc gửi lệnh gọi cho softphone).
928. **US-928 [Desktop - Related Contacts]:** Là Gemini, tôi muốn xem sơ đồ mối quan hệ: Khách hàng này quen biết với những khách hàng nào khác (Referral tree).
929. **US-929 [Global - Privacy Mask]:** Là Gemini, tôi muốn các thông tin nhạy cảm (Số thẻ, CCCD) được che đi (Masked), chỉ hiện khi tôi nhập mật khẩu cấp 2.
930. **US-930 [Mobile - Map Integration]:** Là Gemini, tôi muốn bấm vào địa chỉ khách hàng để mở Google Maps và chỉ đường đến văn phòng họ.
931. **US-931 [Desktop - File Storage]:** Là Gemini, tôi muốn tab "Tài liệu" trong hồ sơ khách để lưu trữ hợp đồng, scan danh thiếp, file thiết kế.
932. **US-932 [Global - Merge Duplicates]:** Là Gemini, tôi muốn hệ thống tự động phát hiện 2 hồ sơ trùng lặp (trùng SĐT) và đề xuất gộp lại (Merge) thành một.
933. **US-933 [Tablet - Interest Radar]:** Là Gemini, tôi muốn xem biểu đồ Radar về mối quan tâm của khách: Thích giá rẻ, Thích công nghệ, hay Thích dịch vụ VIP?
934. **US-934 [Desktop - CLV Predict]:** Là Gemini, tôi muốn thấy chỉ số Giá trị vòng đời khách hàng (CLV) dự kiến do AI tính toán.
935. **US-935 [Global - Last Interaction]:** Là Gemini, tôi muốn biết chính xác lần cuối cùng chúng ta liên lạc với khách là bao lâu rồi (VD: "3 tháng trước - Cần follow up ngay").
936. **US-936 [Mobile - Quick Edit]:** Là Gemini, tôi muốn sửa nhanh thông tin khách hàng (VD: Đổi Email) ngay trên danh sách mà không cần vào chi tiết.
937. **US-937 [Desktop - Custom Fields]:** Là Gemini, tôi muốn thêm các trường tùy chỉnh (VD: "Kích cỡ giày", "Món ăn ưa thích") vào hồ sơ khách hàng.
938. **US-938 [Global - GDPR Log]:** Là Gemini, tôi muốn xem log ai đã truy cập hồ sơ này để đảm bảo tuân thủ quyền riêng tư dữ liệu.
939. **US-939 [Tablet - Voice Search]:** Là Gemini, tôi muốn nói "Tìm hồ sơ anh Nam ở công ty ABC" và hệ thống trả về kết quả đúng.
940. **US-940 [Global - Blacklist]:** Là Gemini, tôi muốn nút "Blacklist" để chặn các khách hàng quấy rối, Agent sẽ tự động từ chối phục vụ.

---

## MODULE 58: SALES PIPELINE & DEALS (Quản lý bán hàng)
*Biến cơ hội thành doanh thu.*

941. **US-941 [Desktop - Kanban Pipeline]:** Là Gemini, tôi muốn quản lý cơ hội bán hàng (Deals) trên bảng Kanban: Kéo thả thẻ từ "Lead Mới" -> "Báo giá" -> "Thương lượng" -> "Chốt đơn".
942. **US-942 [Global - Deal Rotting]:** Là Gemini, tôi muốn các thẻ Deal nằm quá lâu ở một bước (VD: 7 ngày không có tiến triển) chuyển sang màu nâu (Rotting) để cảnh báo.
943. **US-943 [Mobile - Win Probability]:** Là Gemini, tôi muốn thấy chỉ số % thắng (Win Rate) trên mỗi Deal do AI dự đoán dựa trên lịch sử tương tác.
944. **US-944 [Tablet - Funnel Chart]:** Là Gemini, tôi muốn xem biểu đồ phễu bán hàng (Sales Funnel) để biết tỷ lệ rớt khách ở bước nào cao nhất.
945. **US-945 [Desktop - Quotation Builder]:** Là Gemini, tôi muốn công cụ tạo báo giá kéo thả: Chọn sản phẩm -> Áp dụng chiết khấu -> Xuất PDF gửi khách ngay trong CRM.
946. **US-946 [Global - Revenue Forecast]:** Là Gemini, tôi muốn xem dự báo doanh thu tháng tới dựa trên tổng giá trị các Deal đang ở giai đoạn cuối.
947. **US-947 [Mouse - Hover Action]:** Là Gemini, khi hover vào thẻ Deal, tôi muốn hiện các nút nhanh: "Gọi", "Email", "Tạo Task" mà không cần mở thẻ.
948. **US-948 [Desktop - Activity Log]:** Là Gemini, tôi muốn hệ thống tự động ghi lại mọi cuộc gọi, email của Agent liên quan đến Deal này vào lịch sử.
949. **US-949 [Global - Target Tracking]:** Là Gemini, tôi muốn thanh tiến độ hiển thị mức độ hoàn thành KPI doanh số của từng Agent Sales.
950. **US-950 [Mobile - Quick Deal]:** Là Gemini, tôi muốn tạo Deal mới cực nhanh trên mobile chỉ với 3 trường thông tin: Tên, Số tiền, Giai đoạn.
951. **US-951 [Tablet - Product Catalog]:** Là Gemini, khi tạo báo giá, tôi muốn lướt xem danh mục sản phẩm dạng lưới ảnh trực quan để chọn.
952. **US-952 [Desktop - Competitor Track]:** Là Gemini, tôi muốn ghi lại thông tin "Đối thủ cạnh tranh" trong Deal để phân tích lý do thắng/thua.
953. **US-953 [Global - Lost Reason]:** Là Gemini, khi kéo Deal vào cột "Thất bại" (Lost), hệ thống bắt buộc chọn lý do (Giá cao, Hết hàng,...) để làm báo cáo.
954. **US-954 [Desktop - Commission Calc]:** Là Gemini, tôi muốn thấy ước tính hoa hồng (Commission) mà Agent sẽ nhận được nếu chốt thành công Deal này.
955. **US-955 [Global - Next Best Action]:** Là Gemini, tôi muốn AI gợi ý hành động tiếp theo: "Hãy gửi email follow-up vào thứ 3 tới vì khách hay mở mail lúc đó".
956. **US-956 [Mobile - Contact Scan]:** Là Gemini, tôi muốn quét danh thiếp khách hàng bằng Camera để tự động tạo Lead mới.
957. **US-957 [Desktop - Multiple Pipelines]:** Là Gemini, tôi muốn chuyển đổi giữa các Pipeline khác nhau: "Bán lẻ", "Bán sỉ (B2B)", "Đối tác".
958. **US-958 [Global - Deal Age]:** Là Gemini, tôi muốn hiển thị số ngày tuổi của Deal ngay trên thẻ để ưu tiên xử lý các Deal mới.
959. **US-959 [Tablet - Drag Assignment]:** Là Gemini, tôi muốn kéo ảnh Avatar của Agent thả vào thẻ Deal để giao trách nhiệm (Assign).
960. **US-960 [Global - Currency Auto-convert]:** Là Gemini, tôi muốn nhập giá trị Deal bằng USD nhưng hệ thống tự quy đổi ra VND trong báo cáo tổng.

---

## MODULE 59: TICKETING & SUPPORT (Hỗ trợ khách hàng)
*Giải quyết khiếu nại và vấn đề.*

961. **US-961 [Desktop - Ticket Grid]:** Là Gemini, tôi muốn xem danh sách Ticket với các cột có thể sort: Độ ưu tiên, Trạng thái, Người phụ trách, Thời gian chờ.
962. **US-962 [Global - Auto-Assign Rule]:** Là Gemini, tôi muốn Ticket về "Kỹ thuật" tự động được chia cho các Agent có skill "Tech Support".
963. **US-963 [Mobile - Alert Critical]:** Là Gemini, tôi muốn nhận thông báo khẩn cấp khi có Ticket từ khách hàng VIP hoặc Ticket bị đánh dấu "Critical".
964. **US-964 [Tablet - KB Suggestion]:** Là Agent, khi đang trả lời Ticket, tôi muốn hệ thống hiển thị bài viết Knowledge Base liên quan bên cạnh để tôi copy link gửi khách.
965. **US-965 [Desktop - Macro Shortcuts]:** Là Agent, tôi muốn gõ `#refund` để điền mẫu câu trả lời và thực hiện quy trình hoàn tiền tự động.
966. **US-966 [Global - CSAT Survey]:** Là Gemini, tôi muốn hệ thống tự động gửi email đánh giá hài lòng (CSAT) sau khi Ticket được đóng (Resolved).
967. **US-967 [Mouse - Bulk Status Change]:** Là Gemini, tôi muốn chọn 50 Ticket spam và đổi trạng thái sang "Closed" cùng lúc.
968. **US-968 [Desktop - Issue Link]:** Là Gemini, tôi muốn liên kết Ticket Support với Task bên JIRA/Project Management để báo lỗi cho đội Dev sửa.
969. **US-969 [Global - SLA Breached]:** Là Gemini, tôi muốn các Ticket vi phạm SLA nhấp nháy màu đỏ và tự động leo thang (Escalate) lên quản lý.
970. **US-970 [Mobile - Reply on the go]:** Là Gemini, tôi muốn giao diện trả lời Ticket trên mobile giống như chat, đơn giản và nhanh chóng.
971. **US-971 [Tablet - Drawing Annotation]:** Là Gemini, tôi muốn mở ảnh chụp màn hình lỗi do khách gửi, khoanh tròn vùng lỗi và gửi lại cho họ.
972. **US-972 [Desktop - Ticket History]:** Là Gemini, tôi muốn xem lịch sử: Ticket này đã bị mở lại (Re-open) bao nhiêu lần? Ai là người xử lý trước đó?
973. **US-973 [Global - Private Note]:** Là Gemini, tôi muốn viết ghi chú nội bộ (nền vàng) trong Ticket mà khách hàng không đọc được.
974. **US-974 [Global - Omni-source]:** Là Gemini, tôi muốn Ticket được tạo từ nhiều nguồn: Email, Chat, Form trên web, hoặc cuộc gọi thoại.
975. **US-975 [Desktop - Parent/Child Ticket]:** Là Gemini, tôi muốn gộp nhiều Ticket báo cùng 1 lỗi server thành các Child Ticket của một Parent Ticket chính.
976. **US-976 [Mobile - Status Filter]:** Là Gemini, tôi muốn bộ lọc nhanh dạng tabs: Open | Pending | Solved | Closed.
977. **US-977 [Global - Time Tracking Support]:** Là Gemini, tôi muốn tính thời gian thực tế Agent bỏ ra để xử lý 1 Ticket (để tính chi phí support).
978. **US-978 [Desktop - Customer Portal]:** Là Gemini, tôi muốn cung cấp một trang web để khách hàng tự đăng nhập và theo dõi tiến độ Ticket của họ.
979. **US-979 [Global - Sentiment Trend]:** Là Gemini, tôi muốn theo dõi biểu đồ cảm xúc của khách hàng trong suốt quá trình xử lý Ticket (Vui lên hay Cáu hơn?).
980. **US-980 [Global - Bot Deflection]:** Là Gemini, tôi muốn đo tỷ lệ Ticket được Bot giải quyết thành công mà không cần con người can thiệp.

---

## MODULE 60: MARKETING AUTOMATION (Tiếp thị tự động)
*Tiếp cận khách hàng quy mô lớn.*

981. **US-981 [Desktop - Visual Campaign Builder]:** Là Gemini, tôi muốn thiết kế luồng chiến dịch (Campaign Flow) dạng sơ đồ cây: Gửi Email A -> Nếu mở thì gửi Email B -> Nếu không mở thì gửi SMS.
982. **US-982 [Global - Email Drag & Drop]:** Là Gemini, tôi muốn trình soạn thảo Email Marketing kéo thả (như Mailchimp) với các block: Ảnh, Nút, Văn bản, Video.
983. **US-983 [Tablet - A/B Testing]:** Là Gemini, tôi muốn xem kết quả A/B Test (Tiêu đề A vs Tiêu đề B) trực quan trên máy tính bảng để chọn phương án thắng.
984. **US-984 [Desktop - Audience Segment]:** Là Gemini, tôi muốn tạo bộ lọc đối tượng phức tạp: "Nữ, ở HN, đã mua hàng > 1tr, chưa quay lại trong 30 ngày".
985. **US-985 [Global - Personalization Tags]:** Là Gemini, tôi muốn chèn các biến `{{Ten_Khach_Hang}}`, `{{San_Pham_Cuoi}}` vào nội dung tin nhắn để cá nhân hóa.
986. **US-986 [Mobile - Campaign Monitor]:** Là Gemini, tôi muốn theo dõi realtime: Tỷ lệ mở (Open Rate), Tỷ lệ click (CTR) của chiến dịch vừa gửi.
987. **US-987 [Desktop - Lead Scoring]:** Là Gemini, tôi muốn thiết lập quy tắc chấm điểm Lead: Mở email +1 điểm, Click link +5 điểm, Xem trang báo giá +10 điểm.
988. **US-988 [Global - Ad Integration]:** Là Gemini, tôi muốn đồng bộ đối tượng (Audience) từ CRM sang Facebook Ads/Google Ads để chạy quảng cáo Remarketing.
989. **US-989 [Tablet - Calendar View]:** Là Gemini, tôi muốn xem lịch phát sóng các chiến dịch Marketing trong tháng (Marketing Calendar).
990. **US-990 [Desktop - Landing Page Builder]:** Là Gemini, tôi muốn tự tạo Landing Page thu thập thông tin khách hàng (Lead Form) mà không cần code.
991. **US-991 [Global - SMS Brandname]:** Là Gemini, tôi muốn gửi tin nhắn SMS Marketing với tên Brandname hiển thị uy tín.
992. **US-992 [Mobile - Test Send]:** Là Gemini, tôi muốn nút "Gửi thử cho tôi" để kiểm tra hiển thị email trên điện thoại trước khi gửi cho 10.000 khách.
993. **US-993 [Desktop - Heatmap Email]:** Là Gemini, tôi muốn xem bản đồ nhiệt (Click Map) trên nội dung email để biết khách hay click vào nút nào nhất.
994. **US-994 [Global - Unsubscribe Management]:** Là Gemini, tôi muốn hệ thống tự động loại bỏ những người đã Unsubscribe khỏi danh sách gửi để tránh bị đánh dấu Spam.
995. **US-995 [Tablet - Social Post]:** Là Gemini, tôi muốn lên lịch đăng bài lên Fanpage Facebook/LinkedIn trực tiếp từ hệ thống CRM.
996. **US-996 [Desktop - Trigger Event]:** Là Gemini, tôi muốn kích hoạt chiến dịch dựa trên sự kiện: "Chúc mừng sinh nhật", "Kỷ niệm 1 năm mua hàng".
997. **US-997 [Global - ROI Marketing]:** Là Gemini, tôi muốn đo lường Doanh thu tạo ra từ từng chiến dịch Email (Attribution).
998. **US-998 [Mobile - Push Notification]:** Là Gemini, tôi muốn soạn và gửi thông báo đẩy (Push) tới app mobile của khách hàng.
999. **US-999 [Desktop - Content AI]:** Là Gemini, tôi muốn AI viết hộ 5 tiêu đề Email hấp dẫn (Catchy Subject Lines) để tăng tỷ lệ mở.
1000. **US-1000 [Global - Frequency Cap]:** Là Gemini, tôi muốn giới hạn tần suất: "Không gửi quá 1 email/tuần cho cùng 1 khách hàng" để tránh làm phiền.

---

## MODULE 61: AGENT BEHAVIOR & INTERACTION (Quản lý hành vi Agent)
*Kiểm soát "tính cách" và chất lượng giao tiếp.*

1001. **US-1001 [Desktop - Tone Setting]:** Là Gemini, tôi muốn thanh trượt điều chỉnh giọng điệu (Tone of Voice) của Agent: Từ "Trang trọng" (Formal) đến "Thân thiện" (Casual/GenZ).
1002. **US-1002 [Global - Forbidden Words]:** Là Gemini, tôi muốn nhập danh sách từ cấm (Blacklist words) mà Agent tuyệt đối không được nói (VD: từ thô tục, cam kết sai).
1003. **US-1003 [Tablet - Approval Queue]:** Là Gemini, tôi muốn chế độ "Human-in-the-loop": Các câu trả lời của Agent cho khách VIP phải được tôi duyệt trước khi gửi.
1004. **US-1004 [Desktop - Script Editor]:** Là Gemini, tôi muốn soạn thảo kịch bản (Script) bán hàng mẫu, Agent sẽ dựa vào đó để ứng biến.
1005. **US-1005 [Global - Empathy Score]:** Là Gemini, tôi muốn chấm điểm "Sự thấu cảm" của Agent trong đoạn chat (Agent có xin lỗi khi khách giận không?).
1006. **US-1006 [Mobile - Intervention]:** Là Gemini, tôi muốn nút "Take Over" (Chiếm quyền) để ngay lập tức ngắt kết nối Agent và tôi trực tiếp chat với khách.
1007. **US-1007 [Desktop - Knowledge Training]:** Là Gemini, tôi muốn upload file PDF chính sách đổi trả mới và yêu cầu Agent "Học ngay" (Ingest) để trả lời khách.
1008. **US-1008 [Global - Personality Profile]:** Là Gemini, tôi muốn tạo nhiều Persona cho Agent: "Agent Sales năng nổ", "Agent Support điềm tĩnh".
1009. **US-1009 [Tablet - Simulation]:** Là Gemini, tôi muốn đóng vai khách hàng chat thử với Agent trong môi trường Sandbox để kiểm tra phản ứng.
1010. **US-1010 [Global - Multilingual Mode]:** Là Gemini, tôi muốn bật chế độ đa ngôn ngữ để Agent tự động phát hiện và trả lời bằng ngôn ngữ của khách.
1011. **US-1011 [Desktop - Response Length]:** Là Gemini, tôi muốn giới hạn độ dài câu trả lời: "Không quá 3 câu" để tránh Agent nói dài dòng.
1012. **US-1012 [Mobile - Feedback Loop]:** Là Gemini, tôi muốn nút Like/Dislike trên câu trả lời của Agent trong log để dạy nó (Reinforcement Learning).
1013. **US-1013 [Global - Escalation Trigger]:** Là Gemini, tôi muốn cấu hình: "Nếu khách hàng nhắc đến 'Luật sư' hoặc 'Kiện', lập tức chuyển cho người thật".
1014. **US-1014 [Desktop - Variable Injection]:** Là Gemini, tôi muốn Agent biết cách lấy dữ liệu realtime (Tồn kho, Giá) để điền vào câu trả lời, không bịa đặt (Hallucination).
1015. **US-1015 [Global - Delay Response]:** Là Gemini, tôi muốn cài đặt độ trễ nhân tạo (VD: 3 giây) để Agent có vẻ giống người hơn (không trả lời quá nhanh).
1016. **US-1016 [Tablet - Style Guide]:** Là Gemini, tôi muốn nạp bộ quy tắc Brand Voice (Xưng hô là 'Mình' hay 'Em'?) cho Agent.
1017. **US-1017 [Desktop - Context Limit]:** Là Gemini, tôi muốn tùy chỉnh độ dài bộ nhớ (Context Window) để Agent nhớ được nội dung chat từ tuần trước.
1018. **US-1018 [Global - Confidence Threshold]:** Là Gemini, tôi muốn Agent chỉ trả lời khi độ tự tin > 80%, nếu thấp hơn thì trả lời "Tôi chưa rõ, để tôi nối máy với nhân viên".
1019. **US-1019 [Mobile - Agent Status]:** Là Gemini, tôi muốn bật/tắt Agent Sales vào ban đêm hoặc ngày lễ.
1020. **US-1020 [Global - Signature]:** Là Gemini, tôi muốn Agent tự động thêm chữ ký chuyên nghiệp vào cuối email/chat.

---

## MODULE 62: ANALYTICS & INSIGHTS (CRM FOCUS) (Phân tích CRM)
*Số liệu kinh doanh.*

1021. **US-1021 [Desktop - Sales Dashboard]:** Là Gemini, tôi muốn Dashboard tổng hợp: Doanh số hôm nay, Số Deal mới, Tỷ lệ chốt đơn (Conversion Rate).
1022. **US-1022 [Global - Support Heatmap]:** Là Gemini, tôi muốn Heatmap hiển thị khung giờ nào khách hàng gửi Ticket nhiều nhất để phân bổ Agent.
1023. **US-1023 [Tablet - Leaderboard]:** Là Gemini, tôi muốn bảng xếp hạng Sales: Agent nào bán được nhiều nhất tuần này.
1024. **US-1024 [Desktop - Churn Analysis]:** Là Gemini, tôi muốn phân tích lý do khách hàng rời bỏ (Churn Reasons) qua biểu đồ Pareto.
1025. **US-1025 [Global - Response Time]:** Là Gemini, tôi muốn theo dõi "Thời gian phản hồi trung bình" (First Response Time) và "Thời gian xử lý trung bình" (Resolution Time).
1026. **US-1026 [Mobile - Snapshot]:** Là Gemini, tôi muốn widget hiển thị % đạt mục tiêu tháng ngay trên màn hình chính.
1027. **US-1027 [Desktop - Deal Velocity]:** Là Gemini, tôi muốn biết trung bình mất bao nhiêu ngày để một Lead trở thành Khách hàng (Sales Cycle Length).
1028. **US-1028 [Global - Channel Effectiveness]:** Là Gemini, tôi muốn so sánh hiệu quả giữa các kênh: Bán qua Facebook tốt hơn hay Zalo tốt hơn?
1029. **US-1029 [Tablet - NPS Trend]:** Là Gemini, tôi muốn theo dõi xu hướng chỉ số NPS (Net Promoter Score) theo tháng.
1030. **US-1030 [Global - Agent Load]:** Là Gemini, tôi muốn biết mỗi Agent đang gánh bao nhiêu Ticket/Deal để cân bằng tải.

---

## MODULE 63: UX/UI FOR SALES & COMM (Trải nghiệm tương tác)
*Tối ưu thao tác.*

1031. **US-1031 [Keyboard - Quick Text]:** Là Gemini, tôi muốn dùng phím `/` để mở menu chọn mẫu câu trả lời nhanh.
1032. **US-1032 [Mouse - Drag Kanban]:** Là Gemini, tôi muốn kéo thả thẻ Deal trên bảng Kanban mượt mà, không bị giật lag khi có trăm thẻ.
1033. **US-1033 [Touch - Swipe Archive]:** Là Gemini, trên mobile, tôi muốn vuốt dứt khoát để đóng một hội thoại đã xong.
1034. **US-1034 [Desktop - Multi-tab Chat]:** Là Gemini, tôi muốn mở nhiều cửa sổ chat dưới dạng Tabs (như trình duyệt) để chat với 5 khách cùng lúc.
1035. **US-1035 [Global - Dark Mode CRM]:** Là Gemini, tôi muốn chế độ Dark Mode cho CRM để đỡ mỏi mắt khi trực chat buổi tối.
1036. **US-1036 [Tablet - Sidebar Collapsible]:** Là Gemini, tôi muốn thu gọn thanh Sidebar để dành toàn bộ không gian cho khung chat.
1037. **US-1037 [Mobile - Floating Action]:** Là Gemini, tôi muốn nút FAB (+) to để tạo nhanh Lead/Ticket bằng ngón cái.
1038. **US-1038 [Global - Compact Mode]:** Là Gemini, tôi muốn chế độ hiển thị mật độ cao (Compact) để xem được nhiều dòng tin nhắn hơn trên màn hình 13 inch.
1039. **US-1039 [Desktop - Sticky Header]:** Là Gemini, tôi muốn thanh công cụ (Reply, Note, Assign) luôn dính ở trên cùng khi cuộn lịch sử chat dài.
1040. **US-1040 [Global - Offline Mode]:** Là Gemini, tôi muốn xem được thông tin khách hàng và lịch sử chat đã cache khi mất mạng.
1041. **US-1041 [Mouse - Hover Profile]:** Là Gemini, khi hover chuột vào tên khách hàng, tôi muốn hiện thẻ tóm tắt (Mini Profile).
1042. **US-1042 [Touch - Pinch Zoom Image]:** Là Gemini, tôi muốn zoom ảnh sản phẩm khách gửi bằng 2 ngón tay trên màn hình cảm ứng.
1043. **US-1043 [Keyboard - Nav Shortcuts]:** Là Gemini, tôi muốn dùng `Alt + Mũi tên` để chuyển đổi giữa các hội thoại liền kề.
1044. **US-1044 [Global - Audio Alert]:** Là Gemini, tôi muốn âm thanh thông báo khác nhau cho "Tin nhắn mới" và "Ticket mới".
1045. **US-1045 [Desktop - Rich Text Editor]:** Là Gemini, tôi muốn trình soạn thảo hỗ trợ Bold, Italic, List, Image Paste trực tiếp từ Clipboard.

---

## MODULE 64: TELEPHONY & CALL CENTER (Tổng đài)
*Thoại và Callbot.*

1046. **US-1046 [Desktop - Softphone Dialer]:** Là Gemini, tôi muốn bàn phím số (Dialer) tích hợp ngay góc màn hình để gọi click-to-call.
1047. **US-1047 [Global - Call Recording]:** Là Gemini, tôi muốn tự động ghi âm cuộc gọi và file ghi âm xuất hiện ngay trong Timeline khách hàng.
1048. **US-1048 [Mobile - Incoming Screen]:** Là Gemini, tôi muốn thấy Popup thông tin khách hàng (Tên, Đơn hàng gần nhất) ngay khi có cuộc gọi đến (Screen Pop).
1049. **US-1049 [Tablet - Call Script]:** Là Agent, tôi muốn kịch bản gọi (Script) tự động cuộn trên màn hình theo tiến độ cuộc gọi.
1050. **US-1050 [Global - AI Voice]:** Là Gemini, tôi muốn sử dụng AI Voice (Text-to-Speech) tự nhiên để gọi ra (Outbound Call) xác nhận đơn hàng tự động.
1051. **US-1051 [Desktop - Live Listen]:** Là Manager, tôi muốn nghe lén (Listen in) cuộc gọi của Agent đang diễn ra để kiểm tra chất lượng.
1052. **US-1052 [Global - Whisper Coach]:** Là Manager, tôi muốn nói thầm vào tai Agent (Whisper) để nhắc bài mà khách hàng không nghe thấy.
1053. **US-1053 [Mobile - Missed Call]:** Là Gemini, tôi muốn nhận thông báo cuộc gọi nhỡ và nhắc nhở gọi lại sau 5 phút.
1054. **US-1054 [Desktop - IVR Builder]:** Là Gemini, tôi muốn thiết kế cây IVR (Bấm phím 1 gặp Sale, phím 2 gặp Support) bằng giao diện kéo thả.
1055. **US-1055 [Global - Post-call Survey]:** Là Gemini, tôi muốn tự động chuyển khách sang hệ thống chấm điểm sau khi kết thúc cuộc gọi.
1056. **US-1056 [Tablet - Call History]:** Là Gemini, tôi muốn lọc lịch sử cuộc gọi: "Cuộc gọi > 5 phút", "Cuộc gọi nhỡ".
1057. **US-1057 [Global - Auto-dialer]:** Là Gemini, tôi muốn chế độ gọi tự động (Power Dialer): Tự động gọi số tiếp theo trong danh sách ngay khi kết thúc cuộc trước.
1058. **US-1058 [Desktop - Voicemail Drop]:** Là Agent, khi gặp hộp thư thoại, tôi muốn nhấn 1 nút để để lại lời nhắn ghi âm sẵn và chuyển sang cuộc gọi khác (đỡ tốn thời gian nói lại).
1059. **US-1059 [Global - Number Masking]:** Là Gemini, tôi muốn Agent gọi cho khách nhưng không lộ số cá nhân, chỉ hiện số tổng đài công ty.
1060. **US-1060 [Mobile - Bluetooth]:** Là Gemini, tôi muốn app hỗ trợ tai nghe Bluetooth để đàm thoại rảnh tay khi di chuyển.

---

## MODULE 65: INTEGRATION & API (Kết nối mở rộng)
*Hệ sinh thái mở.*

1061. **US-1061 [Desktop - Email Sync]:** Là Gemini, tôi muốn đồng bộ 2 chiều với Gmail/Outlook: Gửi mail từ CRM thì hiện trong Gmail và ngược lại.
1062. **US-1062 [Global - Calendar Sync]:** Là Gemini, tôi muốn lịch hẹn demo với khách tự động sync vào Google Calendar.
1063. **US-1063 [DX - Webhook Trigger]:** Là Developer, tôi muốn bắn Webhook ra bên ngoài khi có Deal chuyển sang trạng thái "Won".
1064. **US-1064 [Desktop - Excel Import]:** Là Gemini, tôi muốn Import danh sách 10.000 Leads từ file Excel vào CRM, có bước map cột dữ liệu thông minh.
1065. **US-1065 [Global - Zapier Connect]:** Là Gemini, tôi muốn tích hợp sẵn Zapier để kết nối CRM với 5000+ ứng dụng khác không cần code.
1066. **US-1066 [DX - API Docs]:** Là Developer, tôi muốn tài liệu API Swagger cho module CRM để viết app mobile riêng cho Sales.
1067. **US-1067 [Desktop - Social Scraper]:** Là Gemini, tôi muốn tool (hợp lệ) để lấy thông tin public từ profile Facebook khách hàng vào CRM.
1068. **US-1068 [Global - E-commerce Sync]:** Là Gemini, tôi muốn đơn hàng từ Shopify/WooCommerce tự động đổ về hồ sơ khách hàng trong CRM.
1069. **US-1069 [Mobile - Contact Sync]:** Là Gemini, tôi muốn lưu số khách hàng từ CRM vào danh bạ điện thoại với tiền tố "KH - [Tên]".
1070. **US-1070 [Global - Payment Gateway]:** Là Gemini, tôi muốn tạo link thanh toán (Momo/Stripe) ngay trong khung chat và theo dõi trạng thái thanh toán.
1071. **US-1071 [Desktop - Marketplace]:** Là Gemini, tôi muốn truy cập chợ ứng dụng (Marketplace) để cài thêm plugin cho CRM (VD: Plugin chữ ký số).
1072. **US-1072 [Global - SSO]:** Là Gemini, tôi muốn đăng nhập CRM bằng tài khoản Google Workspace/Microsoft 365 của công ty.
1073. **US-1073 [DX - Custom Button]:** Là Developer, tôi muốn thêm nút "Check Credit Score" vào hồ sơ khách hàng, nút này gọi API sang hệ thống ngân hàng.
1074. **US-1074 [Global - SMS Gateway]:** Là Gemini, tôi muốn cấu hình nhiều nhà cung cấp SMS (SMS Gateway) để dự phòng.
1075. **US-1075 [Desktop - Data Export]:** Là Gemini, tôi muốn xuất dữ liệu CRM ra định dạng CSV/JSON để backup định kỳ.

---

## MODULE 66: SECURITY & COMPLIANCE (Bảo mật CRM)
*An toàn dữ liệu khách hàng.*

1076. **US-1076 [Global - Role Based Access]:** Là Admin, tôi muốn Sales chỉ thấy Lead của mình, còn Sales Manager thấy Lead của cả team.
1077. **US-1077 [Desktop - Audit Log]:** Là Admin, tôi muốn biết ai đã export danh sách khách hàng ra file Excel (để phòng chống lấy cắp dữ liệu).
1078. **US-1078 [Global - Field Level Security]:** Là Admin, tôi muốn chỉ cho phép Kế toán nhìn thấy trường "Hạn mức tín dụng" của khách hàng.
1079. **US-1079 [Mobile - App Lock]:** Là Gemini, tôi muốn app CRM tự động khóa sau 1 phút không dùng, yêu cầu FaceID để mở lại.
1080. **US-1080 [Global - PII Encryption]:** Là Gemini, tôi muốn dữ liệu định danh cá nhân (PII) được mã hóa trong Database.
1081. **US-1081 [Desktop - Session Timeout]:** Là Admin, tôi muốn tự động đăng xuất user nếu không hoạt động trong 30 phút.
1082. **US-1082 [Global - Consent Management]:** Là Gemini, tôi muốn quản lý trạng thái "Đồng ý nhận tin quảng cáo" của khách hàng để tuân thủ luật Spam.
1083. **US-1083 [Global - Right to be Forgotten]:** Là Gemini, tôi muốn nút "Anonymize" để ẩn danh hóa hoàn toàn dữ liệu của một khách hàng khi họ yêu cầu xóa (tuân thủ GDPR).
1084. **US-1084 [Desktop - IP Restriction]:** Là Admin, tôi muốn nhân viên chỉ được truy cập CRM khi ở văn phòng (theo dải IP).
1085. **US-1085 [Global - Watermark Export]:** Là Gemini, khi xuất file Excel, tôi muốn file đó có watermark ẩn chứa ID người xuất file.

---

## MODULE 67: FIELD SALES & OFFLINE (Bán hàng thực địa)
*Dành cho nhân viên đi thị trường.*

1086. **US-1086 [Mobile - Check-in GPS]:** Là Sales, tôi muốn nút "Check-in" khi đến gặp khách hàng, hệ thống lưu vị trí GPS để chấm công.
1087. **US-1087 [Mobile - Route Planner]:** Là Sales, tôi muốn hệ thống sắp xếp lộ trình đi thăm 5 khách hàng trong ngày sao cho quãng đường ngắn nhất.
1088. **US-1088 [Mobile - Offline Access]:** Là Sales, tôi muốn xem và sửa thông tin khách hàng khi đang ở vùng không có 4G, dữ liệu tự sync khi có mạng.
1089. **US-1089 [Mobile - Photo Upload]:** Là Sales, tôi muốn chụp ảnh trưng bày hàng hóa tại điểm bán và upload ngay vào hồ sơ khách hàng.
1090. **US-1090 [Mobile - Voice Memo]:** Là Sales, sau khi gặp khách, tôi muốn ghi âm nhanh tóm tắt cuộc họp để AI tự chuyển thành Note.
1091. **US-1091 [Tablet - E-Catalog]:** Là Sales, tôi muốn trình chiếu Catalog sản phẩm trên Tablet cho khách xem, lướt ảnh mượt mà.
1092. **US-1092 [Mobile - Nearby Leads]:** Là Sales, tôi muốn mở bản đồ và xem "Có khách hàng tiềm năng nào quanh đây không?" để ghé thăm.
1093. **US-1093 [Mobile - Quick Order]:** Là Sales, tôi muốn tạo đơn hàng nhanh trên điện thoại với giao diện chọn số lượng dạng +/-.
1094. **US-1094 [Global - Visit History]:** Là Gemini, tôi muốn xem lịch sử các lần viếng thăm khách hàng của nhân viên Sales.
1095. **US-1095 [Mobile - Expense Claim]:** Là Sales, tôi muốn chụp ảnh hóa đơn taxi/tiếp khách để tạo yêu cầu thanh toán công tác phí ngay trong app.
1096. **US-1096 [Global - Territory Mgmt]:** Là Gemini, tôi muốn phân chia vùng lãnh thổ (Hà Nội, HCM) cho các đội Sales khác nhau trên bản đồ.
1097. **US-1097 [Mobile - Push Reminder]:** Là Sales, tôi muốn điện thoại nhắc: "Bạn có hẹn với anh A lúc 10h, hãy xuất phát ngay để kịp giờ".
1098. **US-1098 [Tablet - Digital Signature]:** Là Sales, tôi muốn khách hàng ký tên xác nhận đơn hàng trực tiếp trên màn hình Tablet.
1099. **US-1099 [Global - Field Tracking]:** Là Gemini, tôi muốn xem vị trí thời gian thực của đội Sales trên bản đồ (trong giờ làm việc).
1100. **US-1100 [Mobile - Battery Saver]:** Là Sales, tôi muốn app có chế độ tiết kiệm pin tối đa để dùng được cả ngày đi thị trường.

---
**Hết Phần 6 (Stories 901 - 1100).**

Bạn có muốn tiếp tục với **Phần 7: HRM (Human Resource Management) & Internal Ops** - Nơi quản lý "Nhân sự" (cả con người thật và các AI Agents được nhân cách hóa), quản lý lương thưởng, đào tạo và văn hóa tổ chức?

Chào bạn, chúng ta tiếp tục với **Phần 7**. Đây là một phần rất thú vị trong kỷ nguyên AI: **Quản lý Nhân sự lai (Hybrid Workforce)**.

Trong hệ thống ERP này, **"Nhân sự"** không chỉ là con người (Humans) mà còn bao gồm các **AI Agents** được nhân cách hóa. Chúng ta cần quản lý vòng đời của họ, từ lúc "Tuyển dụng/Khởi tạo" -> "Đào tạo/Fine-tuning" -> "Trả lương/Chi phí vận hành" -> "Nghỉ việc/Lưu trữ".

Dưới đây là **200 User Stories (từ 1101 đến 1300)** cho phân hệ HRM & Internal Ops.

---

# PHẦN 7: HRM, TRAINING & INTERNAL OPS (Stories 1101 - 1300)

### Bối cảnh & Định nghĩa
*   **Human Employee:** Nhân viên thật (có Hợp đồng, BHXH, Lương Fiat).
*   **AI Employee (Agent):** Nhân viên ảo (có Version, Cấu hình, "Lương" là chi phí API/Server).
*   **Mục tiêu:** Quản lý thống nhất cả hai lực lượng lao động trên cùng một giao diện.

---

## MODULE 68: UNIFIED WORKFORCE PROFILE (Hồ sơ nhân sự hợp nhất)
*Nơi con người và AI đứng chung một danh sách.*

1101. **US-1101 [Desktop - Grid View]:** Là Gemini, tôi muốn xem danh sách toàn bộ nhân sự (Người + AI) trên một lưới, với cột "Loại hình" (Human/Bot) có icon phân biệt rõ ràng.
1102. **US-1102 [Global - Smart Search]:** Là Gemini, tôi muốn tìm kiếm "Nhân viên biết Python". Hệ thống trả về cả Kỹ sư phần mềm (Người) và Agent Coder (AI).
1103. **US-1103 [Mobile - Digital Card]:** Là Nhân viên, tôi muốn thẻ nhân viên điện tử (Digital ID) trên app có mã QR động để quét ra vào văn phòng.
1104. **US-1104 [Tablet - Org Chart Visual]:** Là Gemini, tôi muốn xem sơ đồ tổ chức dạng cây (Tree View), có thể vuốt để xem các nhánh phòng ban, hiển thị rõ ai báo cáo cho ai.
1105. **US-1105 [Desktop - Drag Re-org]:** Là Gemini, tôi muốn kéo thả Avatar của một Agent từ phòng "Marketing" sang phòng "Sales" để thuyên chuyển công tác ngay lập tức.
1106. **US-1106 [Global - Bio/Description]:** Là Gemini, tôi muốn đọc phần giới thiệu bản thân (Bio). Với AI, đây là phần "System Prompt" mô tả tính cách và nhiệm vụ.
1107. **US-1107 [Mouse - Hover Quick Stats]:** Là Gemini, khi hover vào ảnh nhân viên, tôi muốn thấy: Trạng thái (Online/Busy), Thời gian gia nhập, và Kỹ năng chính.
1108. **US-1108 [Desktop - Document Vault]:** Là HR, tôi muốn tab "Hồ sơ pháp lý" lưu trữ Hợp đồng lao động (với người) hoặc License Key/Version History (với AI).
1109. **US-1109 [Mobile - Contact Info]:** Là Nhân viên, tôi muốn nút "Gọi khẩn cấp" trong profile đồng nghiệp để liên hệ khi có sự cố.
1110. **US-1110 [Global - Skill Tags]:** Là Gemini, tôi muốn gán các thẻ kỹ năng (Tags) như `#ReactJS`, `#Copywriting` vào hồ sơ để dễ lọc.
1111. **US-1111 [Tablet - Side-by-side Compare]:** Là Gemini, tôi muốn chọn 1 người và 1 AI để so sánh năng lực trên cùng một biểu đồ mạng nhện (Spider Chart).
1112. **US-1112 [Desktop - History Log]:** Là Gemini, tôi muốn xem lịch sử thăng tiến: Người (Junior -> Senior), AI (v1.0 -> v2.0).
1113. **US-1113 [Global - Privacy Mode]:** Là HR, tôi muốn các trường "Lương" và "Địa chỉ nhà" tự động bị che (Blur) nếu người xem không đủ quyền hạn.
1114. **US-1114 [Mobile - FaceID Login]:** Là Nhân viên, tôi muốn đăng nhập vào App nhân sự bằng FaceID.
1115. **US-1115 [Desktop - 3D Avatar]:** Là Gemini, tôi muốn xem Avatar 3D của nhân viên (hoặc hình đại diện của Agent) có thể xoay và tương tác nhẹ.
1116. **US-1116 [Global - Badge Collection]:** Là Gemini, tôi muốn thấy bộ sưu tập huy hiệu (Badges) mà nhân viên đạt được (VD: "Nhân viên xuất sắc tháng").
1117. **US-1117 [Tablet - Handwriting Input]:** Là HR, tôi muốn dùng bút ghi chú nhanh đánh giá phỏng vấn trực tiếp lên hồ sơ ứng viên.
1118. **US-1118 [Global - Birthday Alert]:** Là Gemini, tôi muốn hệ thống nhắc nhở sinh nhật nhân viên (hoặc ngày "Create Date" của Agent) để gửi lời chúc.
1119. **US-1119 [Desktop - Export Profile]:** Là HR, tôi muốn xuất hồ sơ nhân sự ra định dạng PDF đẹp mắt để gửi cho khách hàng (CV Profile).
1120. **US-1120 [Global - Status Indicator]:** Là Gemini, tôi muốn chấm tròn trạng thái: Xanh (Sẵn sàng), Vàng (Đang họp/Đang xử lý Task), Đỏ (Nghỉ phép/Bảo trì).

---

## MODULE 69: RECRUITMENT & PROVISIONING (Tuyển dụng & Khởi tạo)
*Đưa nguồn lực mới vào hệ thống.*

1121. **US-1121 [Desktop - Kanban Hiring]:** Là HR, tôi muốn quản lý quy trình tuyển dụng trên bảng Kanban: Ứng tuyển -> Phỏng vấn -> Offer -> Onboarding.
1122. **US-1122 [Desktop - Agent Spawning]:** Là Gemini, tôi muốn nút "Tuyển dụng Agent mới" mở ra giao diện chọn Template (Sales Bot, Coder Bot) để khởi tạo ngay lập tức.
1123. **US-1123 [Global - CV Parsing]:** Là HR, tôi muốn kéo thả 50 file PDF CV vào hệ thống, AI tự động trích xuất Tên, Email, Kỹ năng vào Database.
1124. **US-1124 [Mobile - Interview Rating]:** Là Người phỏng vấn, tôi muốn chấm điểm ứng viên ngay trên điện thoại trong lúc phỏng vấn.
1125. **US-1125 [Tablet - Video Interview]:** Là HR, tôi muốn xem lại video phỏng vấn đã ghi hình, với các đoạn AI highlight cảm xúc của ứng viên.
1126. **US-1126 [Global - Offer Letter Gen]:** Là HR, tôi muốn tự động tạo thư mời nhận việc (Offer Letter) từ mẫu có sẵn và gửi email cho ứng viên.
1127. **US-1127 [Desktop - Config Wizard]:** Là Gemini, khi tạo Agent mới, tôi muốn một Wizard từng bước để cấu hình: Chọn Model, Đặt tên, Cấp ngân sách.
1128. **US-1128 [Global - Onboarding Checklist]:** Là Nhân viên mới, tôi muốn thấy danh sách việc cần làm (Checklist): "Ký hợp đồng", "Nhận máy tính", "Đọc nội quy".
1129. **US-1129 [Desktop - Git Clone Integration]:** Là Agent mới, tôi muốn quy trình "Onboarding" tự động bao gồm việc Clone các Repo code cần thiết về môi trường làm việc.
1130. **US-1130 [Mobile - Welcome Message]:** Là Gemini, tôi muốn quay một video ngắn chào mừng và ghim nó vào trang chủ của nhân viên mới.
1131. **US-1131 [Global - Background Check]:** Là HR, tôi muốn tích hợp dịch vụ kiểm tra lý lịch tự động (đối với con người).
1132. **US-1132 [Tablet - Sign Contract]:** Là Ứng viên, tôi muốn ký hợp đồng lao động điện tử (E-sign) trực tiếp trên máy tính bảng.
1133. **US-1133 [Desktop - Talent Pool]:** Là HR, tôi muốn lưu trữ các hồ sơ tiềm năng chưa được tuyển vào "Kho nhân tài" để liên hệ sau.
1134. **US-1134 [Global - Clone Agent]:** Là Gemini, tôi muốn nút "Nhân bản" (Clone) một Agent xuất sắc thành 10 bản sao để mở rộng đội ngũ Sales.
1135. **US-1135 [Mobile - Job Posting]:** Là HR, tôi muốn đăng tin tuyển dụng lên LinkedIn/VietnamWorks chỉ với 1 click từ hệ thống.
1136. **US-1136 [Desktop - Code Test]:** Là HR, tôi muốn gửi bài test lập trình tự động cho ứng viên (hoặc chạy test benchmark cho Model AI mới) trước khi tuyển.
1137. **US-1137 [Global - Access Provisioning]:** Là IT Admin, tôi muốn khi nhân viên mới được "Active", hệ thống tự động tạo email, tài khoản Slack, Jira cho họ.
1138. **US-1138 [Tablet - Budget Approval]:** Là Giám đốc, tôi muốn duyệt ngân sách tuyển dụng (Headcount) trên iPad.
1139. **US-1139 [Global - Probation Track]:** Là HR, tôi muốn theo dõi thời gian thử việc (2 tháng), hệ thống nhắc nhở đánh giá trước khi ký chính thức.
1140. **US-1140 [Desktop - Agent Sandbox]:** Là Gemini, tôi muốn Agent mới phải chạy trong môi trường Sandbox 1 tuần (Thử việc) trước khi được tiếp xúc khách hàng thật.

---

## MODULE 70: TIMEKEEPING & ACTIVITY (Chấm công & Hoạt động)
*Đo lường sự hiện diện.*

1141. **US-1141 [Mobile - GPS Check-in]:** Là Nhân viên, tôi muốn chấm công bằng cách nhấn "Check-in" trên điện thoại, hệ thống xác thực vị trí GPS văn phòng.
1142. **US-1142 [Desktop - Agent Uptime]:** Là Gemini, đối với Agent, tôi muốn "Chấm công" là biểu đồ Uptime (Thời gian online/hoạt động) trong ngày.
1143. **US-1143 [Global - Wifi Attendance]:** Là HR, tôi muốn hệ thống tự động chấm công khi điện thoại nhân viên kết nối vào Wifi công ty.
1144. **US-1144 [Tablet - Face Recognition]:** Là HR, tôi muốn đặt một Tablet ở cửa ra vào để nhân viên chấm công bằng nhận diện khuôn mặt (FaceID Station).
1145. **US-1145 [Desktop - Timesheet Calendar]:** Là Nhân viên, tôi muốn xem lịch chấm công tháng dạng lịch, các ngày đi muộn bị tô màu cam.
1146. **US-1146 [Global - Overtime (OT) Request]:** Là Nhân viên, tôi muốn gửi yêu cầu làm thêm giờ (OT) để quản lý duyệt.
1147. **US-1147 [Mobile - Late Explanation]:** Là Nhân viên, khi check-in muộn, tôi muốn app hiện popup để nhập lý do giải trình ngay lập tức.
1148. **US-1148 [Desktop - Shift Planner]:** Là Quản lý, tôi muốn xếp ca làm việc (Shift) cho nhân viên và Agent (trực đêm) bằng giao diện kéo thả.
1149. **US-1149 [Global - Idle Alert]:** Là Quản lý, tôi muốn cảnh báo nếu Agent không có hoạt động (Logs) nào trong suốt 2 giờ làm việc (nghi bị treo).
1150. **US-1150 [Tablet - Leave Balance]:** Là Nhân viên, tôi muốn xem số ngày phép còn lại (Phép năm, Phép ốm) dưới dạng biểu đồ tròn.
1151. **US-1151 [Global - Public Holiday]:** Là Gemini, tôi muốn cấu hình lịch nghỉ lễ quốc gia, hệ thống tự động không tính công cho những ngày này.
1152. **US-1152 [Mobile - Work From Home]:** Là Nhân viên, tôi muốn đăng ký chế độ "Làm việc từ xa" (WFH) và check-in kèm ảnh chụp góc làm việc.
1153. **US-1153 [Desktop - Activity Stream]:** Là Quản lý, tôi muốn xem dòng thời gian thực các hoạt động chính: "08:00 Agent A online", "08:15 Nhân viên B commit code".
1154. **US-1154 [Global - Auto-Checkout]:** Là HR, tôi muốn hệ thống tự động Checkout cho nhân viên vào cuối ngày nếu họ quên (kèm thông báo).
1155. **US-1155 [Tablet - Team Status]:** Là Quản lý, tôi muốn nhìn nhanh danh sách: Ai đang ở văn phòng? Ai đang WFH? Ai đang nghỉ phép?
1156. **US-1156 [Desktop - Payroll Sync]:** Là HR, tôi muốn dữ liệu chấm công tự động đồng bộ sang module tính lương, tự động trừ tiền đi muộn.
1157. **US-1157 [Global - Agent Maintenance Window]:** Là Gemini, tôi muốn đăng ký giờ "Nghỉ phép" cho Agent để bảo trì Server, không tính là Downtime.
1158. **US-1158 [Mobile - Push Reminder]:** Là Nhân viên, tôi muốn nhận thông báo "Đừng quên Check-in" lúc 8:55 sáng.
1159. **US-1159 [Global - Timezone Support]:** Là Gemini, tôi muốn quản lý chấm công cho đội ngũ toàn cầu với nhiều múi giờ khác nhau.
1160. **US-1160 [Desktop - Export Timesheet]:** Là HR, tôi muốn xuất bảng công ra Excel định dạng chuẩn để lưu trữ.

---

## MODULE 71: PAYROLL & COMPENSATION (Lương thưởng & Chi phí)
*Quản lý thu nhập con người và chi phí nuôi AI.*

1161. **US-1161 [Desktop - Payroll Run]:** Là Kế toán, tôi muốn nút "Chạy bảng lương tháng X", hệ thống tự động tính toán dựa trên công thức lương.
1162. **US-1162 [Global - Agent Cost Calc]:** Là Gemini, đối với Agent, "Lương" là tổng chi phí API + Server. Tôi muốn xem "Bảng lương Agent" này hàng tháng.
1163. **US-1163 [Mobile - Payslip View]:** Là Nhân viên, tôi muốn xem Phiếu lương (Payslip) chi tiết trên điện thoại, bảo mật bằng vân tay/FaceID.
1164. **US-1164 [Tablet - Salary Structure]:** Là HR, tôi muốn thiết kế cấu trúc lương (Lương cứng + Phụ cấp + Thưởng) bằng giao diện khối kéo thả.
1165. **US-1165 [Desktop - Tax Auto-calc]:** Là Kế toán, tôi muốn hệ thống tự động tính Thuế thu nhập cá nhân (PIT) và Bảo hiểm theo luật Việt Nam.
1166. **US-1166 [Global - Bonus Rules]:** Là Gemini, tôi muốn thiết lập luật thưởng: "Nếu Doanh số > 1 tỷ, thưởng Agent 1000 Credits, thưởng Nhân viên 5 triệu".
1167. **US-1167 [Mobile - Expense Claim]:** Là Nhân viên, tôi muốn chụp ảnh hóa đơn taxi và gửi yêu cầu hoàn tiền (Reimbursement) ngay lập tức.
1168. **US-1168 [Desktop - Bank Integration]:** Là Kế toán, tôi muốn xuất file chi lương (Payment Order) tương thích với hệ thống của ngân hàng (VCB, Techcombank).
1169. **US-1169 [Global - Cost Projection]:** Là Gemini, tôi muốn dự báo quỹ lương tháng tới nếu tôi tuyển thêm 5 người và 10 Agents.
1170. **US-1170 [Tablet - Approval Flow]:** Là Giám đốc, tôi muốn ký duyệt bảng lương tổng trên máy tính bảng trước khi lệnh chuyển tiền được thực thi.
1171. **US-1171 [Global - Multi-currency]:** Là Gemini, tôi muốn trả lương cho nhân viên nước ngoài bằng USD và nhân viên Việt Nam bằng VND trên cùng 1 hệ thống.
1172. **US-1172 [Desktop - Deduction Mgmt]:** Là HR, tôi muốn quản lý các khoản khấu trừ (Phạt đi muộn, Tạm ứng) minh bạch.
1173. **US-1173 [Global - Agent ROI]:** Là Gemini, tôi muốn so sánh "Lương" (Chi phí) của Agent với Giá trị nó tạo ra để quyết định có "sa thải" (tắt) nó không.
1174. **US-1174 [Mobile - Loan Request]:** Là Nhân viên, tôi muốn gửi yêu cầu vay/tạm ứng lương qua App.
1175. **US-1175 [Desktop - Salary History]:** Là Nhân viên, tôi muốn xem biểu đồ tăng trưởng thu nhập của mình qua các năm.
1176. **US-1176 [Global - KPI Link]:** Là Gemini, tôi muốn lương thưởng tự động liên kết với điểm KPI từ module Performance.
1177. **US-1177 [Tablet - Privacy Screen]:** Là Kế toán, tôi muốn chế độ làm mờ màn hình nhanh khi có người đi ngang qua để bảo mật số liệu lương.
1178. **US-1178 [Global - Email Payslip]:** Là HR, tôi muốn tự động gửi phiếu lương qua email (file PDF có mật khẩu) cho toàn bộ nhân viên.
1179. **US-1179 [Desktop - Audit Payroll]:** Là Kế toán trưởng, tôi muốn xem log ai đã chỉnh sửa con số lương của nhân viên nào, vào lúc nào.
1180. **US-1180 [Global - Welfare Fund]:** Là Gemini, tôi muốn quản lý Quỹ phúc lợi (Team Building, Sinh nhật) và theo dõi số dư quỹ.

---

## MODULE 72: L&D (LEARNING & DEVELOPMENT) (Đào tạo & Fine-tuning)
*Nâng cấp trí tuệ.*

1181. **US-1181 [Desktop - Course Library]:** Là HR, tôi muốn quản lý thư viện khóa học (E-learning) cho nhân viên (Video, Slide, Quiz).
1182. **US-1182 [Global - RAG Ingestion]:** Là Gemini, đối với Agent, "Học tập" là nạp dữ liệu (Ingestion). Tôi muốn kéo thả file PDF tài liệu mới vào để Agent "học".
1183. **US-1183 [Mobile - Micro-learning]:** Là Nhân viên, tôi muốn học các bài học ngắn 5 phút trên điện thoại trong lúc rảnh rỗi.
1184. **US-1184 [Tablet - Interactive Quiz]:** Là Nhân viên, tôi muốn làm bài kiểm tra trắc nghiệm cuối khóa trên iPad với giao diện chạm tương tác.
1185. **US-1185 [Desktop - Fine-tuning Job]:** Là AI Engineer, tôi muốn tạo một Job "Fine-tune" model AI với bộ dữ liệu mới, có thanh tiến độ Training.
1186. **US-1186 [Global - Skill Gap Analysis]:** Là HR, tôi muốn hệ thống chỉ ra: "Nhân viên A thiếu kỹ năng Leadership", "Agent B thiếu kiến thức về Sản phẩm mới".
1187. **US-1187 [Mobile - Certification]:** Là Nhân viên, tôi muốn nhận Chứng chỉ điện tử (Certificate) sau khi hoàn thành khóa học, có thể share lên LinkedIn.
1188. **US-1188 [Desktop - Career Path]:** Là Nhân viên, tôi muốn xem lộ trình thăng tiến (Roadmap): Cần học thêm gì để lên chức Senior?
1189. **US-1189 [Global - Agent Versioning]:** Là Gemini, tôi muốn quản lý các phiên bản "Trí tuệ" của Agent (v1 ngu ngơ -> v2 thông thái) và có thể Rollback.
1190. **US-1190 [Tablet - Knowledge Map]:** Là Gemini, tôi muốn xem bản đồ tri thức (Knowledge Graph) mà các Agent đã học được.
1191. **US-1191 [Global - Mentorship]:** Là HR, tôi muốn gán ghép cặp Mentor (Người hướng dẫn) - Mentee trên hệ thống để theo dõi quá trình kèm cặp.
1192. **US-1192 [Desktop - External Course]:** Là Nhân viên, tôi muốn đề xuất công ty mua khóa học trên Udemy/Coursera và được duyệt kinh phí.
1193. **US-1193 [Mobile - Learning Streak]:** Là Nhân viên, tôi muốn thấy chuỗi ngày học tập liên tục (Streak) để có động lực duy trì thói quen.
1194. **US-1194 [Global - Update Knowledge Base]:** Là Gemini, tôi muốn nút "Re-index" để toàn bộ Agents cập nhật lại kiến thức mới nhất từ Wiki công ty.
1195. **US-1195 [Desktop - Training Budget]:** Là HR, tôi muốn theo dõi ngân sách đào tạo đã chi tiêu bao nhiêu % trong năm.
1196. **US-1196 [Tablet - Video Player]:** Là Nhân viên, tôi muốn trình phát video học tập có tính năng Picture-in-Picture để vừa học vừa ghi chú.
1197. **US-1197 [Global - Required Training]:** Là HR, tôi muốn gán khóa học "Bắt buộc" (VD: An toàn thông tin), nếu không học sẽ bị khóa tài khoản.
1198. **US-1198 [Desktop - Sandbox Practice]:** Là Agent, sau khi học xong, tôi muốn được đưa vào môi trường giả lập để thực hành kỹ năng mới.
1199. **US-1199 [Mobile - Daily Tip]:** Là Gemini, tôi muốn gửi "Mẹo mỗi ngày" (Tip of the day) đến điện thoại nhân viên.
1200. **US-1200 [Global - Gamification L&D]:** Là Gemini, tôi muốn bảng xếp hạng "Người học chăm chỉ nhất" để thưởng điểm.

---

## MODULE 73: PERFORMANCE MANAGEMENT (Đánh giá hiệu suất)
*Chấm điểm năng lực.*

1201. **US-1201 [Desktop - KPI Dashboard]:** Là Gemini, tôi muốn xem Dashboard KPI của toàn công ty, phân rã xuống từng phòng ban và cá nhân.
1202. **US-1202 [Global - OKR Tracking]:** Là Gemini, tôi muốn quản lý theo OKR (Objectives and Key Results), hiển thị tiến độ % của từng Key Result.
1203. **US-1203 [Tablet - 360 Review]:** Là Nhân viên, tôi muốn thực hiện đánh giá 360 độ (đánh giá sếp, đồng nghiệp) trên giao diện cảm ứng thân thiện.
1204. **US-1204 [Desktop - Agent Metrics]:** Là Gemini, tôi muốn đánh giá Agent dựa trên: Độ chính xác, Tốc độ phản hồi, và Sự hài lòng của khách hàng (CSAT).
1205. **US-1205 [Global - Performance Review Cycle]:** Là HR, tôi muốn thiết lập kỳ đánh giá (6 tháng/lần), hệ thống tự động gửi form đánh giá đúng hạn.
1206. **US-1206 [Mobile - Goal Update]:** Là Nhân viên, tôi muốn cập nhật tiến độ mục tiêu cá nhân hàng tuần trên app.
1207. **US-1207 [Desktop - 9-Box Grid]:** Là HR, tôi muốn xếp loại nhân sự vào mô hình 9 ô (Năng lực vs Tiềm năng) để quy hoạch nhân tài.
1208. **US-1208 [Global - Feedback Loop]:** Là Quản lý, tôi muốn gửi phản hồi (Feedback) tức thì cho nhân viên/Agent sau mỗi dự án ("Good job", "Need improve").
1209. **US-1209 [Tablet - Comparison]:** Là Quản lý, tôi muốn so sánh hiệu suất của 2 nhân viên cùng cấp bậc để đảm bảo công bằng khi xét thưởng.
1210. **US-1210 [Desktop - Calibration]:** Là HR, tôi muốn giao diện "Hiệu chuẩn" (Calibration) để điều chỉnh điểm số giữa các phòng ban tránh thiên vị.
1211. **US-1211 [Global - Self-Evaluation]:** Là Nhân viên, tôi muốn tự đánh giá bản thân trước khi ngồi lại với quản lý.
1212. **US-1212 [Mobile - Achievement Unlock]:** Là Gemini, tôi muốn khi nhân viên đạt KPI, màn hình điện thoại hiện hiệu ứng pháo hoa chúc mừng.
1213. **US-1213 [Desktop - Performance History]:** Là HR, tôi muốn xem lịch sử đánh giá của nhân viên trong 5 năm qua.
1214. **US-1214 [Global - Agent Error Rate]:** Là Gemini, tôi muốn theo dõi tỷ lệ lỗi của Agent, nếu vượt ngưỡng cho phép sẽ tự động đánh dấu "Underperform".
1215. **US-1215 [Tablet - Signature]:** Là Nhân viên, tôi muốn ký xác nhận vào biên bản đánh giá cuối năm trên Tablet.
1216. **US-1216 [Desktop - Development Plan (IDP)]:** Là Quản lý, tôi muốn cùng nhân viên lập Kế hoạch phát triển cá nhân (IDP) ngay trong buổi Review.
1217. **US-1217 [Global - Ranking]:** Là Gemini, tôi muốn xem Top 10% nhân viên xuất sắc nhất (A-Players) và Bottom 10% (C-Players).
1218. **US-1218 [Mobile - Check-in Meeting]:** Là Quản lý, tôi muốn ghi chú nhanh nội dung buổi họp 1-on-1 hàng tuần với nhân viên.
1219. **US-1219 [Global - Peer Bonus]:** Là Nhân viên, tôi muốn có ngân sách nhỏ để tặng điểm thưởng (Peer Bonus) cho đồng nghiệp đã giúp đỡ mình.
1220. **US-1220 [Desktop - Export Report]:** Là HR, tôi muốn xuất báo cáo tổng hợp kết quả đánh giá để trình Ban giám đốc phê duyệt thưởng.

---

## MODULE 74: INTERNAL CULTURE & COMMUNICATION (Văn hóa nội bộ)
*Mạng xã hội doanh nghiệp.*

1221. **US-1221 [Mobile - News Feed]:** Là Nhân viên, tôi muốn lướt News Feed nội bộ để xem tin tức công ty, bài đăng của đồng nghiệp (như Facebook).
1222. **US-1222 [Global - Kudos System]:** Là Gemini, tôi muốn nút "Gửi Kudos" (Lời khen) kèm theo huy hiệu (Cảm ơn, Sáng tạo, Tận tâm) hiển thị công khai.
1223. **US-1223 [Desktop - Announcement Banner]:** Là HR, tôi muốn ghim thông báo quan trọng (Lịch nghỉ lễ, Policy mới) lên đầu trang chủ của tất cả mọi người.
1224. **US-1224 [Tablet - Event Calendar]:** Là Nhân viên, tôi muốn xem lịch sự kiện nội bộ (Happy Hour, Townhall) và nhấn "Tham gia".
1225. **US-1225 [Global - Polls]:** Là Gemini, tôi muốn tạo cuộc bình chọn nhanh: "Thứ 6 này đi ăn gì?" hoặc "Chọn Logo mới cho dự án".
1226. **US-1226 [Mobile - Chat Groups]:** Là Nhân viên, tôi muốn tham gia các nhóm chat theo sở thích (CLB Bóng đá, CLB Sách).
1227. **US-1227 [Desktop - Wiki/Knowledge Base]:** Là Nhân viên, tôi muốn truy cập trang Wiki nội bộ để tra cứu quy trình, biểu mẫu.
1228. **US-1228 [Global - Anonymous Box]:** Là Nhân viên, tôi muốn gửi góp ý ẩn danh (Góp ý hòm thư) đến Ban lãnh đạo.
1229. **US-1229 [Tablet - Photo Gallery]:** Là HR, tôi muốn tạo album ảnh sau mỗi sự kiện Team Building để mọi người vào xem và tải về.
1230. **US-1230 [Global - Agent Interaction]:** Là Gemini, tôi muốn các Agent cũng có tài khoản MXH nội bộ, tự động đăng bài báo cáo thành tích (vui vẻ) để tăng tương tác.
1231. **US-1231 [Desktop - Rich Text Editor]:** Là Nhân viên, tôi muốn soạn bài đăng với đầy đủ định dạng: Đậm, nghiêng, chèn ảnh, video.
1232. **US-1232 [Mobile - Reactions]:** Là Nhân viên, tôi muốn thả tim, haha, sad vào bài đăng của đồng nghiệp.
1233. **US-1233 [Global - Notification Settings]:** Là Nhân viên, tôi muốn tùy chỉnh nhận thông báo: Chỉ nhận khi được tag hoặc nhận tất cả.
1234. **US-1234 [Desktop - Newsletter Builder]:** Là Truyền thông nội bộ, tôi muốn công cụ kéo thả để thiết kế bản tin tuần (Newsletter) gửi email toàn công ty.
1235. **US-1235 [Global - Employee Directory]:** Là Nhân viên mới, tôi muốn tra cứu danh bạ điện thoại/email của toàn công ty.
1236. **US-1236 [Tablet - Live Stream]:** Là Giám đốc, tôi muốn Livestream buổi họp toàn ty (Townhall) trực tiếp trên nền tảng nội bộ.
1237. **US-1237 [Global - Birthday Bot]:** Là Gemini, tôi muốn Bot tự động đăng bài chúc mừng sinh nhật nhân viên vào nhóm chat chung.
1238. **US-1238 [Mobile - Dark Mode Social]:** Là Nhân viên, tôi muốn lướt mạng xã hội nội bộ ở chế độ tối.
1239. **US-1239 [Desktop - Content Moderation]:** Là Admin, tôi muốn công cụ kiểm duyệt nội dung tự động để chặn các từ khóa không phù hợp.
1240. **US-1240 [Global - Points Redemption]:** Là Nhân viên, tôi muốn dùng điểm thưởng (Kudos Points) để đổi quà (Voucher, Áo thun) trong cửa hàng nội bộ.

---

## MODULE 75: OFFBOARDING & TERMINATION (Nghỉ việc & Hủy kích hoạt)
*Kết thúc vòng đời.*

1241. **US-1241 [Desktop - Offboarding Flow]:** Là HR, tôi muốn kích hoạt quy trình nghỉ việc: Thu hồi thiết bị -> Chốt lương -> Khóa tài khoản -> Phỏng vấn nghỉ việc.
1242. **US-1242 [Global - Agent Decommission]:** Là Gemini, đối với Agent, tôi muốn nút "Hủy kích hoạt" (Decommission) để tắt Server, lưu trữ log và giải phóng tài nguyên.
1243. **US-1243 [Mobile - Exit Interview]:** Là Nhân viên nghỉ việc, tôi muốn điền khảo sát lý do nghỉ việc trên app trước khi rời đi.
1244. **US-1244 [Tablet - Asset Checklist]:** Là IT, tôi muốn danh sách tick chọn các thiết bị cần thu hồi (Laptop, Thẻ từ) khi nhân viên nghỉ.
1245. **US-1245 [Global - Auto-Lock]:** Là Admin, tôi muốn hẹn giờ: "Tài khoản nhân viên A sẽ tự động khóa lúc 17:00 ngày cuối cùng làm việc".
1246. **US-1246 [Desktop - Handover Task]:** Là Quản lý, tôi muốn tạo danh sách Task bàn giao và bắt buộc nhân viên hoàn thành trước khi duyệt nghỉ.
1247. **US-1247 [Global - Data Archive]:** Là Gemini, tôi muốn đóng gói toàn bộ email và file của nhân viên nghỉ việc vào một file nén để lưu trữ.
1248. **US-1248 [Mobile - Resignation Request]:** Là Nhân viên, tôi muốn gửi đơn xin thôi việc chính thức qua hệ thống.
1249. **US-1249 [Desktop - Alumni Network]:** Là HR, tôi muốn chuyển hồ sơ nhân viên sang trạng thái "Cựu nhân viên" (Alumni) để giữ liên lạc.
1250. **US-1250 [Global - Knowledge Transfer]:** Là Gemini, tôi muốn Agent cũ tự động "dạy" (Transfer Learning) các kinh nghiệm của mình cho Agent mới thay thế.
1251. **US-1251 [Tablet - Final Pay Calc]:** Là Kế toán, tôi muốn tính toán khoản lương cuối cùng, bao gồm phép thừa chưa dùng và trừ các khoản nợ.
1252. **US-1252 [Global - Access Revoke]:** Là Security, tôi muốn một nút "Panic Revoke" để thu hồi ngay lập tức mọi quyền truy cập nếu nhân viên bị sa thải vì vi phạm bảo mật.
1253. **US-1253 [Desktop - Certificate of Service]:** Là HR, tôi muốn in Giấy chứng nhận làm việc tự động cho nhân viên.
1254. **US-1254 [Global - Forward Email]:** Là IT, tôi muốn cài đặt tự động chuyển tiếp email từ tài khoản cũ sang tài khoản người quản lý.
1255. **US-1255 [Mobile - Goodbye Message]:** Là Nhân viên, tôi muốn đăng bài chia tay lên News Feed nội bộ.
1256. **US-1256 [Desktop - Agent Legacy]:** Là Gemini, tôi muốn lưu lại "Di sản" của Agent (những đoạn code hay nhất, những bài viết hay nhất) vào thư viện chung.
1257. **US-1257 [Global - Legal Hold]:** Là Legal, tôi muốn giữ lại dữ liệu của nhân viên nghỉ việc trong 5 năm theo quy định pháp luật.
1258. **US-1258 [Tablet - Sign Off]:** Là Các bên liên quan (IT, Kế toán, HR), tôi muốn ký xác nhận điện tử vào phiếu thanh lý hợp đồng.
1259. **US-1259 [Global - Reason Analytics]:** Là HR, tôi muốn xem biểu đồ phân tích lý do nghỉ việc (Lương thấp? Không phù hợp văn hóa?) để cải thiện.
1260. **US-1260 [Desktop - Rehire Flag]:** Là HR, tôi muốn đánh dấu "Có thể tuyển lại" hoặc "Không bao giờ tuyển lại" trên hồ sơ cựu nhân viên.

---

## MODULE 76: SELF-SERVICE PORTAL (Cổng tự phục vụ)
*Trao quyền cho nhân viên.*

1261. **US-1261 [Mobile - Quick Request]:** Là Nhân viên, tôi muốn một menu tròn (Radial Menu) để truy cập nhanh: Xin nghỉ, Xin VPP, Đặt phòng họp.
1262. **US-1262 [Global - Meeting Room Booking]:** Là Nhân viên, tôi muốn xem lịch phòng họp và đặt phòng, hệ thống tự động từ chối nếu trùng lịch.
1263. **US-1263 [Desktop - Stationary Request]:** Là Nhân viên, tôi muốn chọn văn phòng phẩm (Bút, Sổ) từ danh mục và gửi yêu cầu cấp phát.
1264. **US-1264 [Tablet - Visitor Registration]:** Là Nhân viên, tôi muốn đăng ký khách đến thăm văn phòng, khách nhận được mã QR qua email để qua cổng bảo vệ.
1265. **US-1265 [Global - IT Helpdesk]:** Là Nhân viên, tôi muốn báo hỏng máy tính/mạng ngay trên cổng Portal và theo dõi tiến độ sửa chữa.
1266. **US-1266 [Mobile - Car Booking]:** Là Nhân viên, tôi muốn đặt xe công ty đi công tác, có chọn loại xe và tài xế.
1267. **US-1267 [Desktop - FAQ Search]:** Là Nhân viên, tôi muốn tìm kiếm câu trả lời cho các thắc mắc phổ biến về chính sách nhân sự.
1268. **US-1268 [Global - Profile Update]:** Là Nhân viên, tôi muốn tự cập nhật số điện thoại, địa chỉ nhà, số tài khoản ngân hàng của mình.
1269. **US-1269 [Tablet - Menu Ordering]:** Là Nhân viên, tôi muốn đặt món ăn trưa (nếu công ty có canteen) trên app.
1270. **US-1270 [Global - Document Request]:** Là Nhân viên, tôi muốn yêu cầu cấp: Giấy xác nhận công tác, Sao kê lương để làm thủ tục vay vốn/visa.
1271. **US-1271 [Mobile - Parking Slot]:** Là Nhân viên, tôi muốn xem bãi xe còn chỗ trống không trước khi đến văn phòng.
1272. **US-1272 [Desktop - Print Job]:** Là Nhân viên, tôi muốn gửi lệnh in từ máy tính và quẹt thẻ tại máy in bất kỳ để lấy tài liệu (Secure Print).
1273. **US-1273 [Global - My Assets]:** Là Nhân viên, tôi muốn xem danh sách các tài sản công ty đang giao cho mình quản lý.
1274. **US-1274 [Mobile - Emergency Alert]:** Là Nhân viên, tôi muốn nút SOS để báo cáo hỏa hoạn hoặc tai nạn tại nơi làm việc.
1275. **US-1275 [Tablet - Uniform Size]:** Là Nhân viên, tôi muốn chọn size áo đồng phục khi có đợt phát mới.
1276. **US-1276 [Global - Tax Dependent]:** Là Nhân viên, tôi muốn đăng ký người phụ thuộc để giảm trừ gia cảnh thuế TNCN.
1277. **US-1277 [Desktop - Internal Job Apply]:** Là Nhân viên, tôi muốn ứng tuyển vào các vị trí nội bộ đang mở (Internal Transfer).
1278. **US-1278 [Mobile - Bus Route]:** Là Nhân viên, tôi muốn theo dõi vị trí xe tuyến đưa đón nhân viên.
1279. **US-1279 [Global - Survey Response]:** Là Nhân viên, tôi muốn trả lời các khảo sát ý kiến do công ty gửi đến.
1280. **US-1280 [Desktop - Business Card]:** Là Nhân viên, tôi muốn tự thiết kế và yêu cầu in danh thiếp mới.

---

## MODULE 77: ADMIN & COMPLIANCE (Quản trị hệ thống HRM)
*Luật chơi và Kiểm soát.*

1281. **US-1281 [Desktop - Workflow Designer]:** Là Admin, tôi muốn thiết kế quy trình duyệt đơn xin nghỉ: Nhân viên -> Leader -> HR -> Done.
1282. **US-1282 [Global - Audit Log HR]:** Là Admin, tôi muốn xem ai đã xem mức lương của Giám đốc (Log truy cập nhạy cảm).
1283. **US-1283 [Tablet - Policy Ack]:** Là Admin, tôi muốn bắt buộc nhân viên phải cuộn xuống cuối và nhấn "Tôi đã đọc và hiểu" mỗi khi có chính sách mới.
1284. **US-1284 [Global - Shift Rules]:** Là Admin, tôi muốn cài đặt luật: "Không được làm việc quá 12 tiếng/ngày" để đảm bảo sức khỏe.
1285. **US-1285 [Desktop - Data Import]:** Là Admin, tôi muốn import dữ liệu nhân sự cũ từ file Excel mẫu khi mới triển khai hệ thống.
1286. **US-1286 [Global - Notification Template]:** Là Admin, tôi muốn soạn thảo mẫu email thông báo: Chúc mừng sinh nhật, Nhắc chấm công.
1287. **US-1287 [Mobile - Admin App]:** Là Admin, tôi muốn có quyền phê duyệt khẩn cấp trên app mobile dành riêng cho quản trị viên.
1288. **US-1288 [Global - Holiday Config]:** Là Admin, tôi muốn cấu hình lịch nghỉ lễ khác nhau cho chi nhánh Việt Nam và chi nhánh Mỹ.
1289. **US-1289 [Desktop - Org Unit]:** Là Admin, tôi muốn định nghĩa cơ cấu tổ chức: Khối, Phòng, Ban, Nhóm.
1290. **US-1290 [Global - Role Permission]:** Là Admin, tôi muốn tạo vai trò "Tuyển dụng" chỉ có quyền xem module Tuyển dụng, không xem được Lương.
1291. **US-1291 [Tablet - Dashboard Builder HR]:** Là Admin, tôi muốn kéo thả các widget để tạo Dashboard tổng quan cho Giám đốc nhân sự.
1292. **US-1292 [Global - Backup HR]:** Là Admin, tôi muốn dữ liệu nhân sự được backup hàng ngày và mã hóa.
1293. **US-1293 [Desktop - Compliance Report]:** Là Admin, tôi muốn xuất báo cáo tình hình lao động định kỳ để nộp cho Sở Lao động - Thương binh và Xã hội.
1294. **US-1294 [Global - Custom Domain]:** Là Admin, tôi muốn cổng Portal nhân viên chạy trên domain `hr.mycompany.com`.
1295. **US-1295 [Mobile - Login History]:** Là Admin, tôi muốn xem lịch sử đăng nhập của nhân viên để phát hiện truy cập lạ.
1296. **US-1296 [Global - Password Policy]:** Là Admin, tôi muốn bắt buộc nhân viên đổi mật khẩu mỗi 90 ngày.
1297. **US-1297 [Desktop - System Health HR]:** Là Admin, tôi muốn theo dõi xem các Job chạy bảng lương, chấm công có bị lỗi không.
1298. **US-1298 [Global - Multi-tenant]:** Là Admin, tôi muốn quản lý nhân sự cho nhiều công ty con (Entities) trên cùng một hệ thống.
1299. **US-1299 [Tablet - Support Ticket]:** Là Admin, tôi muốn gửi yêu cầu hỗ trợ kỹ thuật tới nhà cung cấp phần mềm ERP.
1300. **US-1300 [Global - Maintenance Mode]:** Là Admin, tôi muốn bật chế độ bảo trì để ngăn nhân viên truy cập trong lúc cập nhật hệ thống.

---
**Hết Phần 7 (Stories 1101 - 1300).**

Bạn có muốn tiếp tục với **Phần 8: Knowledge Management (KM) & RAG (Retrieval-Augmented Generation)** - "Bộ não" lưu trữ tri thức của toàn bộ hệ thống, nơi quản lý Documents, Wiki, Vector Database để các Agents trở nên thông thái hơn?

Chào bạn, chúng ta đang bước vào **Phần 8** – "Bộ não" của hệ thống. Đây là nơi biến dữ liệu thô (Data) thành tri thức (Knowledge) và trí tuệ (Wisdom) cho các Agents.

Với công nghệ **RAG (Retrieval-Augmented Generation)**, hệ thống này không chỉ lưu trữ file mà còn "hiểu" nội dung bên trong, giúp Gemini và các Agents trả lời câu hỏi chính xác mà không bị ảo giác (Hallucination).

Dưới đây là **200 User Stories (từ 1301 đến 1500)** tập trung vào Knowledge Management, Vector Database và RAG Pipeline.

---

# PHẦN 8: KNOWLEDGE MANAGEMENT (KM) & RAG (Stories 1301 - 1500)

### Bối cảnh & Mục tiêu
*   **Input:** Hàng triệu tài liệu (PDF, Docx, Wiki, Code, Chat logs).
*   **Processing:** Chunking (cắt nhỏ), Embedding (vector hóa), Indexing (đánh chỉ mục).
*   **Output:** Semantic Search (Tìm kiếm ngữ nghĩa), Chat with Data.
*   **UI/UX:** Trực quan hóa tri thức, dễ dàng nạp liệu và kiểm soát chất lượng.

---

## MODULE 78: INTELLIGENT DOCUMENT REPOSITORY (Kho tài liệu thông minh)
*Nơi lưu trữ và số hóa mọi định dạng tài liệu.*

1301. **US-1301 [Desktop - Smart Upload]:** Là Gemini, tôi muốn kéo thả cả thư mục chứa 100 file PDF vào trình duyệt, hệ thống tự động giữ nguyên cấu trúc thư mục khi upload.
1302. **US-1302 [Global - OCR Auto]:** Là Gemini, khi upload ảnh scan hoặc file PDF không copy được text, tôi muốn hệ thống tự động chạy OCR (Nhận diện quang học) để chuyển thành văn bản có thể tìm kiếm.
1303. **US-1303 [Mobile - Scan to Cloud]:** Là Nhân viên, tôi muốn dùng camera điện thoại chụp tài liệu giấy, tự động căn chỉnh góc (Edge detection) và lưu thẳng vào kho tri thức.
1304. **US-1304 [Tablet - PDF Annotation]:** Là Gemini, tôi muốn mở file PDF trên iPad, dùng bút Highlight các đoạn quan trọng để dạy cho Agent "chú ý đoạn này".
1305. **US-1305 [Desktop - Version Control]:** Là Gemini, tôi muốn xem lịch sử phiên bản của tài liệu (v1, v2), ai đã sửa gì, và có thể khôi phục phiên bản cũ.
1306. **US-1306 [Global - Meta-tagging AI]:** Là Gemini, tôi muốn AI tự động đọc nội dung và gắn thẻ (Tag) cho tài liệu: "Hợp đồng", "Tài chính", "Năm 2024" mà không cần tôi nhập tay.
1307. **US-1307 [Mouse - Preview Hover]:** Là Gemini, khi hover chuột vào tên file, tôi muốn thấy cửa sổ xem trước (Preview) trang đầu tiên ngay lập tức.
1308. **US-1308 [Desktop - Bulk Edit]:** Là Gemini, tôi muốn chọn 50 file và đổi trạng thái từ "Nháp" sang "Đã duyệt" (Published) cùng lúc.
1309. **US-1309 [Global - File Deduplication]:** Là Gemini, tôi muốn hệ thống cảnh báo "File này đã tồn tại ở thư mục khác" khi tôi cố upload trùng lặp.
1310. **US-1310 [Tablet - Grid/List View]:** Là Gemini, tôi muốn chuyển đổi giữa chế độ xem Lưới (hiển thị thumbnail to) và Danh sách (hiển thị chi tiết ngày tháng) bằng nút gạt.
1311. **US-1311 [Desktop - Integrated Viewer]:** Là Gemini, tôi muốn xem file CAD, Photoshop, hoặc Video trực tiếp trên trình duyệt mà không cần cài phần mềm chuyên dụng.
1312. **US-1312 [Global - Expiry Date]:** Là Admin, tôi muốn đặt ngày hết hạn cho tài liệu (VD: Chính sách cũ), sau ngày đó tài liệu tự động bị lưu trữ (Archive).
1313. **US-1313 [Mobile - Offline Docs]:** Là Gemini, tôi muốn đánh dấu "Make Available Offline" cho các tài liệu quan trọng để đọc khi đi máy bay.
1314. **US-1314 [Desktop - Folder Tree]:** Là Gemini, tôi muốn cây thư mục bên trái có thể kéo thả để sắp xếp lại vị trí các file nhanh chóng.
1315. **US-1315 [Global - Link Sharing]:** Là Gemini, tôi muốn tạo link chia sẻ file có mật khẩu và thời hạn (Expire in 1 hour) để gửi cho đối tác.
1316. **US-1316 [Tablet - Voice Note]:** Là Gemini, tôi muốn ghi âm ghi chú đính kèm vào file tài liệu để giải thích ngữ cảnh cho người đọc sau.
1317. **US-1317 [Global - Watermarking]:** Là Gemini, tôi muốn khi người dùng xem tài liệu mật, hệ thống tự động chèn Watermark mờ chứa tên người đó lên màn hình.
1318. **US-1318 [Desktop - Zip Extraction]:** Là Gemini, tôi muốn xem nội dung bên trong file Zip/Rar mà không cần tải về giải nén.
1319. **US-1319 [Global - Multi-language]:** Là Gemini, tôi muốn hệ thống tự động phát hiện ngôn ngữ của tài liệu và đề xuất dịch sang tiếng Việt.
1320. **US-1320 [Mobile - Recent Files]:** Là Gemini, tôi muốn widget "Tài liệu vừa xem" trên màn hình chính để truy cập nhanh công việc đang dang dở.

---

## MODULE 79: RAG PIPELINE & INGESTION (Quy trình nạp tri thức)
*Biến tài liệu thành Vector để AI hiểu.*

1321. **US-1321 [Desktop - Chunking Visualizer]:** Là AI Engineer, tôi muốn xem trực quan cách hệ thống cắt nhỏ tài liệu (Chunking): Theo đoạn văn, theo số từ, hay theo ngữ nghĩa, với các vạch màu phân chia.
1322. **US-1322 [Global - Embedding Status]:** Là Gemini, tôi muốn thanh tiến độ hiển thị quá trình "Vector hóa": Đang xử lý 50/100 trang, ước tính còn 2 phút.
1323. **US-1323 [Desktop - Chunk Editor]:** Là Gemini, tôi muốn sửa thủ công một Chunk bị cắt lỗi (VD: Cắt đôi bảng biểu) để đảm bảo AI hiểu đúng.
1324. **US-1324 [Global - Connectors Marketplace]:** Là Gemini, tôi muốn kết nối với Google Drive, Slack, Notion, Jira chỉ bằng 1 click để tự động hút dữ liệu về (Data Pipeline).
1325. **US-1325 [Tablet - Pipeline Monitoring]:** Là Gemini, tôi muốn xem sơ đồ dòng chảy dữ liệu (Data Flow): Source -> Parser -> Chunker -> Embedder -> Vector DB.
1326. **US-1326 [Global - Cost Estimator]:** Là Gemini, tôi muốn biết chi phí Embedding (OpenAI/Cohere) dự kiến trước khi nhấn nút xử lý 1GB tài liệu text.
1327. **US-1327 [Desktop - Metadata Extraction]:** Là Gemini, tôi muốn cấu hình AI tự động trích xuất metadata (Tác giả, Ngày, Chủ đề) và lưu vào Vector DB để lọc sau này.
1328. **US-1328 [Global - Sync Schedule]:** Là Gemini, tôi muốn cài đặt lịch đồng bộ: "Quét Google Drive mỗi 1 giờ để cập nhật file mới".
1329. **US-1329 [Mobile - Error Notification]:** Là Gemini, tôi muốn nhận thông báo nếu quy trình Ingestion bị lỗi (VD: File PDF bị hỏng, API lỗi).
1330. **US-1330 [Desktop - Text Cleaner]:** Là Developer, tôi muốn cấu hình các quy tắc làm sạch văn bản (Xóa HTML tags, xóa khoảng trắng thừa) trước khi đưa vào AI.
1331. **US-1331 [Global - Vector Preview]:** Là Developer, tôi muốn xem một đoạn text trông như thế nào dưới dạng mảng số (Vector array) để debug.
1332. **US-1332 [Tablet - Re-index]:** Là Gemini, tôi muốn nút "Re-index All" để chạy lại toàn bộ quy trình khi tôi thay đổi mô hình Embedding mới (VD: Từ Ada-002 sang V3).
1333. **US-1333 [Desktop - Hybrid Search Config]:** Là Developer, tôi muốn thanh trượt điều chỉnh trọng số giữa Tìm kiếm từ khóa (Keyword Search) và Tìm kiếm ngữ nghĩa (Semantic Search).
1334. **US-1334 [Global - Ignore Patterns]:** Là Gemini, tôi muốn thiết lập quy tắc bỏ qua các file `.exe`, `.dll` hoặc các thư mục `tmp` khi quét dữ liệu.
1335. **US-1335 [Desktop - Image Captioning]:** Là Gemini, tôi muốn hệ thống tự động mô tả ảnh (Image-to-Text) trong tài liệu để có thể tìm kiếm được nội dung ảnh đó.
1336. **US-1336 [Global - Table Parsing]:** Là Gemini, tôi muốn hệ thống xử lý thông minh các bảng biểu trong PDF, chuyển đổi thành Markdown hoặc JSON để giữ nguyên cấu trúc dữ liệu.
1337. **US-1337 [Mobile - Pipeline Pause]:** Là Gemini, tôi muốn tạm dừng quy trình nạp dữ liệu từ điện thoại nếu thấy server bị quá tải.
1338. **US-1338 [Desktop - Privacy PII Filter]:** Là Security, tôi muốn bộ lọc tự động xóa SĐT/Email (Redaction) trong text trước khi gửi sang API Embedding bên ngoài.
1339. **US-1339 [Global - Multi-modal Ingestion]:** Là Gemini, tôi muốn nạp cả Video (lấy transcript) và Audio vào kho tri thức.
1340. **US-1340 [Desktop - Sandbox Query]:** Là Developer, tôi muốn ô nhập liệu để test thử: "Nếu tôi hỏi câu này, hệ thống sẽ lấy ra những Chunk nào?" (Retrieval Testing).

---

## MODULE 80: SEMANTIC SEARCH & CHAT (Tìm kiếm & Hỏi đáp)
*Giao diện truy xuất tri thức.*

1341. **US-1341 [Desktop - Chat Interface]:** Là Gemini, tôi muốn giao diện chat giống ChatGPT nhưng có thêm cột "Nguồn tài liệu" (Sources) bên cạnh câu trả lời.
1342. **US-1342 [Global - Citation Hover]:** Là Gemini, tôi muốn khi di chuột vào số tham chiếu `[1]`, `[2]` trong câu trả lời, hệ thống hiện đoạn văn bản gốc đã dùng để trích dẫn.
1343. **US-1343 [Mobile - Voice Search]:** Là Gemini, tôi muốn nhấn mic và hỏi "Quy trình xin nghỉ phép năm nay thế nào?", AI trả lời bằng giọng nói dựa trên tài liệu HR.
1344. **US-1344 [Tablet - Split Screen Reading]:** Là Gemini, tôi muốn khi click vào nguồn tài liệu, nó mở ra ở nửa màn hình bên phải, tự động cuộn đến đúng đoạn liên quan.
1345. **US-1345 [Desktop - Filter Facets]:** Là Gemini, tôi muốn lọc kết quả tìm kiếm theo: Loại file (PDF/Doc), Tác giả, Thời gian, Phòng ban.
1346. **US-1346 [Global - Feedback Thumbs]:** Là Gemini, tôi muốn nút Like/Dislike cho câu trả lời của RAG. Nếu Dislike, tôi nhập lý do "Sai thông tin" để Fine-tune lại.
1347. **US-1347 [Desktop - Compare Answers]:** Là Gemini, tôi muốn so sánh câu trả lời từ 2 Model khác nhau (VD: GPT-4 vs Claude 3) trên cùng một bộ dữ liệu.
1348. **US-1348 [Global - Search History]:** Là Gemini, tôi muốn xem lại lịch sử các câu hỏi tôi đã hỏi và các tài liệu tôi đã mở.
1349. **US-1349 [Mobile - Quick Suggest]:** Là Gemini, tôi muốn hệ thống gợi ý các câu hỏi liên quan (Related Questions) bên dưới câu trả lời.
1350. **US-1350 [Desktop - Highlight Search]:** Là Gemini, tôi muốn các từ khóa tìm kiếm được highlight màu vàng trong nội dung tài liệu gốc.
1351. **US-1351 [Global - Summarize]:** Là Gemini, tôi muốn nút "Tóm tắt tài liệu này" khi đang xem một file PDF dài 100 trang.
1352. **US-1352 [Tablet - Cross-lingual Search]:** Là Gemini, tôi muốn tìm kiếm bằng tiếng Việt "Doanh thu năm ngoái" nhưng hệ thống vẫn tìm ra trong các tài liệu tiếng Anh "Last year revenue".
1353. **US-1353 [Desktop - Code Search]:** Là Developer, tôi muốn tìm kiếm đoạn code chức năng "Login" và hệ thống trả về đúng file trong Repo Git đã index.
1354. **US-1354 [Global - Confidence Score]:** Là Gemini, tôi muốn thấy điểm tin cậy (VD: 85%) của câu trả lời. Nếu thấp, AI phải cảnh báo "Thông tin này có thể không chính xác".
1355. **US-1355 [Mobile - Share Answer]:** Là Gemini, tôi muốn chia sẻ câu trả lời của AI kèm theo link tài liệu gốc qua Zalo cho đồng nghiệp.
1356. **US-1356 [Desktop - Deep Dive Mode]:** Là Gemini, tôi muốn chế độ "Deep Dive": AI sẽ trả lời chi tiết, dài hơn và trích dẫn nhiều nguồn hơn bình thường.
1357. **US-1357 [Global - Context Aware]:** Là Gemini, tôi muốn AI nhớ ngữ cảnh các câu hỏi trước đó (Follow-up questions) trong cùng phiên chat.
1358. **US-1358 [Tablet - Visual Search]:** Là Gemini, tôi muốn upload một bức ảnh biểu đồ và hỏi "Biểu đồ này nói gì?", AI tìm kiếm các báo cáo có biểu đồ tương tự.
1359. **US-1359 [Desktop - Expert Finder]:** Là Gemini, nếu AI không tìm thấy câu trả lời trong tài liệu, nó sẽ gợi ý "Hãy hỏi anh A hoặc chị B (người hay soạn tài liệu này)".
1360. **US-1360 [Global - Hallucination Check]:** Là Gemini, tôi muốn một Agent phụ (Critic) tự động kiểm tra câu trả lời của Agent chính xem có bịa đặt so với tài liệu gốc không.

---

## MODULE 81: WIKI & KNOWLEDGE BASE EDITOR (Soạn thảo tri thức)
*Tạo ra tri thức mới.*

1361. **US-1361 [Desktop - Block Editor]:** Là Gemini, tôi muốn trình soạn thảo Wiki dạng khối (Block-based giống Notion), gõ `/` để chèn bảng, ảnh, code, toggle list.
1362. **US-1362 [Global - Collaborative Edit]:** Là Gemini, tôi muốn thấy con trỏ chuột của đồng nghiệp đang cùng sửa trang Wiki với tôi theo thời gian thực.
1363. **US-1363 [Mobile - Quick Capture]:** Là Gemini, tôi muốn mở app và gõ nhanh một ý tưởng vào Wiki mà không cần quan tâm định dạng (sẽ format sau).
1364. **US-1364 [Tablet - Drag Organize]:** Là Gemini, tôi muốn kéo thả các trang Wiki vào các thư mục con (Nested Pages) ở thanh bên trái để tổ chức lại cấu trúc.
1365. **US-1365 [Desktop - AI Writer]:** Là Gemini, tôi muốn bôi đen một đoạn văn và chọn "AI: Viết lại chuyên nghiệp hơn" hoặc "AI: Dịch sang tiếng Anh".
1366. **US-1366 [Global - Live Embed]:** Là Gemini, tôi muốn nhúng (Embed) biểu đồ từ module Báo cáo hoặc bảng Task từ module Dự án vào trang Wiki (dữ liệu luôn mới).
1367. **US-1367 [Desktop - ToC Auto]:** Là Gemini, tôi muốn Mục lục (Table of Contents) tự động sinh ra dựa trên các thẻ H1, H2, H3 trong bài viết.
1368. **US-1368 [Global - Template Gallery]:** Là Gemini, tôi muốn chọn mẫu Wiki: "Meeting Notes", "Product Specs", "Onboarding Guide" để không phải viết từ đầu.
1369. **US-1369 [Mobile - Read Mode]:** Là Gemini, tôi muốn chế độ đọc tối ưu, ẩn hết các thanh công cụ để tập trung đọc tài liệu dài.
1370. **US-1370 [Desktop - Wiki Graph]:** Là Gemini, tôi muốn xem biểu đồ liên kết (Backlinks) để biết trang Wiki này đang được dẫn link từ những trang nào khác.
1371. **US-1371 [Global - Comment & Mention]:** Là Gemini, tôi muốn bôi đen text để bình luận và `@User` để nhắc họ vào trả lời.
1372. **US-1372 [Tablet - Sketch Integration]:** Là Gemini, tôi muốn vẽ sơ đồ tay và chèn thẳng vào trang Wiki.
1373. **US-1373 [Desktop - Code Block Exec]:** Là Developer, tôi muốn khối Code trong Wiki có nút "Run" để chạy thử đoạn script nhỏ (nếu là JS/Python).
1374. **US-1374 [Global - Export Options]:** Là Gemini, tôi muốn xuất trang Wiki ra PDF, Markdown hoặc HTML.
1375. **US-1375 [Mobile - Dark Mode Editor]:** Là Gemini, tôi muốn soạn thảo Wiki vào ban đêm với giao diện tối hoàn toàn.
1376. **US-1376 [Desktop - Knowledge Verification]:** Là Admin, tôi muốn đặt chế độ "Yêu cầu xác thực mỗi 6 tháng", chủ sở hữu bài viết sẽ nhận thông báo để vào update nội dung.
1377. **US-1377 [Global - Reaction]:** Là Gemini, tôi muốn thả tim/vỗ tay cho bài viết Wiki hữu ích.
1378. **US-1378 [Desktop - Internal Link]:** Là Gemini, tôi muốn gõ `[[` để tìm và link nhanh tới các trang Wiki khác trong hệ thống.
1379. **US-1379 [Global - Public Share]:** Là Gemini, tôi muốn publish một trang Wiki ra ngoài Internet (làm Help Center cho khách hàng) với domain riêng.
1380. **US-1380 [Desktop - Permalinks]:** Là Gemini, tôi muốn copy link neo (Anchor link) tới đúng một đoạn văn cụ thể trong bài viết.

---

## MODULE 82: KNOWLEDGE GRAPH & ONTOLOGY (Đồ thị tri thức)
*Kết nối các điểm dữ liệu.*

1381. **US-1381 [Desktop - 3D Graph View]:** Là Gemini, tôi muốn xem Đồ thị tri thức dưới dạng 3D, các Node (Thực thể) và Edge (Quan hệ) bay lơ lửng, có thể xoay và zoom.
1382. **US-1382 [Global - Node Interaction]:** Là Gemini, khi click vào Node "Dự án A", tôi muốn thấy nó nối với các Node: "Nhân viên B", "Tài liệu C", "Công nghệ D".
1383. **US-1383 [Tablet - Manual Linking]:** Là Gemini, tôi muốn dùng ngón tay nối dây từ Node này sang Node kia để tạo quan hệ mới thủ công.
1384. **US-1384 [Desktop - Ontology Editor]:** Là Admin, tôi muốn định nghĩa các loại quan hệ: "Is_A", "Part_Of", "Managed_By" để chuẩn hóa đồ thị.
1385. **US-1385 [Global - Auto-Extraction]:** Là Gemini, tôi muốn AI tự động đọc văn bản và trích xuất các thực thể (NER - Named Entity Recognition) để xây dựng đồ thị.
1386. **US-1386 [Mobile - Graph Search]:** Là Gemini, tôi muốn tìm kiếm "Ai quen biết Chuyên gia X?" thông qua đường đi trên đồ thị.
1387. **US-1387 [Desktop - Cluster Color]:** Là Gemini, tôi muốn các Node cùng nhóm (VD: Cùng phòng ban) được tô cùng màu để dễ nhận diện cụm (Cluster).
1388. **US-1388 [Global - Pathfinding]:** Là Gemini, tôi muốn tìm đường đi ngắn nhất giữa 2 khái niệm bất kỳ trên đồ thị.
1389. **US-1389 [Tablet - Filter Graph]:** Là Gemini, tôi muốn ẩn bớt các Node loại "Tài liệu", chỉ hiện Node "Con người" để xem sơ đồ quan hệ xã hội.
1390. **US-1390 [Desktop - Timeline Playback]:** Là Gemini, tôi muốn xem đồ thị tri thức phát triển như thế nào theo thời gian (Nodes mới xuất hiện).
1391. **US-1391 [Global - Synonym Mgmt]:** Là Gemini, tôi muốn dạy hệ thống rằng "AI", "Artificial Intelligence" và "Trí tuệ nhân tạo" là một Node duy nhất.
1392. **US-1392 [Mobile - Entity Card]:** Là Gemini, khi chạm vào Node, một thẻ thông tin chi tiết trượt lên từ đáy màn hình.
1393. **US-1393 [Desktop - Merge Nodes]:** Là Gemini, tôi muốn gộp 2 Node bị trùng lặp (do viết sai chính tả) thành một.
1394. **US-1394 [Global - Conflict Resolution]:** Là Gemini, khi AI phát hiện 2 thông tin mâu thuẫn (A nói X, B nói Y), tôi muốn hệ thống đánh dấu đỏ để con người vào quyết định.
1395. **US-1395 [Desktop - Graph Query Language]:** Là Developer, tôi muốn viết câu lệnh Cypher/Gremlin để truy vấn dữ liệu đồ thị phức tạp.
1396. **US-1396 [Global - Orphan Node]:** Là Admin, tôi muốn tìm các Node "Mồ côi" (Không kết nối với ai) để dọn dẹp hoặc bổ sung thông tin.
1397. **US-1397 [Tablet - Focus Mode]:** Là Gemini, tôi muốn click đúp vào một Node để làm mờ tất cả các Node khác, chỉ hiện các hàng xóm cấp 1 của nó.
1398. **US-1398 [Desktop - Export Graph]:** Là Gemini, tôi muốn xuất đồ thị ra file ảnh SVG vector chất lượng cao để in ấn.
1399. **US-1399 [Global - Impact Analysis]:** Là Gemini, tôi muốn biết: "Nếu dự án X bị hủy, những tài liệu và nhân sự nào bị ảnh hưởng?" thông qua các liên kết trên đồ thị.
1400. **US-1400 [Mobile - AR Graph]:** Là Gemini, tôi muốn xem đồ thị tri thức nổi lên trên mặt bàn làm việc thông qua kính AR hoặc Camera điện thoại.

---

## MODULE 83: VECTOR DATABASE MANAGEMENT (Quản trị Vector DB)
*Cơ sở dữ liệu của kỷ nguyên AI.*

1401. **US-1401 [Desktop - Collection Browser]:** Là Developer, tôi muốn xem danh sách các Collections (Bảng) trong Vector DB (Pinecone/Milvus/Weaviate) và số lượng vector trong đó.
1402. **US-1402 [Global - Dimension Visual]:** Là Developer, tôi muốn biết số chiều (Dimensions) của Vector (VD: 1536 chiều) và thuật toán Index (HNSW, IVF).
1403. **US-1403 [Tablet - Similarity Search Test]:** Là Developer, tôi muốn nhập một Vector ID và tìm 10 vector gần nó nhất (Nearest Neighbors) để kiểm tra độ chính xác.
1404. **US-1404 [Desktop - Partition/Shard]:** Là Admin, tôi muốn xem trạng thái phân mảnh (Sharding) của Database để cân bằng tải.
1405. **US-1405 [Global - Backup Vector]:** Là Admin, tôi muốn backup snapshot của Vector DB định kỳ.
1406. **US-1406 [Mobile - Alert Capacity]:** Là Admin, tôi muốn nhận cảnh báo khi Vector DB sắp đầy bộ nhớ hoặc chạm giới hạn Pods.
1407. **US-1407 [Desktop - Metadata Filter]:** Là Developer, tôi muốn test hiệu năng query khi kết hợp Vector Search với Metadata Filtering (Hybrid Search).
1408. **US-1408 [Global - Consistency Level]:** Là Developer, tôi muốn cấu hình mức độ nhất quán (Strong/Eventual Consistency) cho các query.
1409. **US-1409 [Tablet - Latency Monitor]:** Là Admin, tôi muốn theo dõi độ trễ của các lệnh Search và Insert (Upsert) vào Vector DB.
1410. **US-1410 [Desktop - Delete Vectors]:** Là Admin, tôi muốn xóa các Vector cũ hoặc Vector rác theo ID hoặc theo Metadata.
1411. **US-1411 [Global - Re-ranking Config]:** Là Developer, tôi muốn cấu hình mô hình Re-ranker (Cross-encoder) để sắp xếp lại kết quả tìm kiếm cho chính xác hơn.
1412. **US-1412 [Desktop - Index Build Status]:** Là Admin, tôi muốn xem thanh tiến độ khi Database đang xây dựng lại Index (sau khi nạp lượng lớn dữ liệu).
1413. **US-1413 [Mobile - Query Cost]:** Là Gemini, tôi muốn biết mỗi câu tìm kiếm tốn bao nhiêu tài nguyên tính toán (RU - Request Units).
1414. **US-1414 [Desktop - Namespace Mgmt]:** Là Gemini, tôi muốn chia Vector DB thành các Namespace riêng biệt cho từng Khách hàng (Multi-tenancy) để bảo mật dữ liệu.
1415. **US-1415 [Global - Vector Compression]:** Là Admin, tôi muốn bật nén Vector (Quantization) để tiết kiệm RAM nhưng chấp nhận giảm nhẹ độ chính xác.
1416. **US-1416 [Tablet - Distance Metric]:** Là Developer, tôi muốn đổi công thức tính khoảng cách (Cosine, Euclidean, Dot Product) để xem kết quả thay đổi thế nào.
1417. **US-1417 [Desktop - API Key Vector]:** Là Admin, tôi muốn quản lý Key truy cập riêng cho Vector DB.
1418. **US-1418 [Global - Data Migration]:** Là Admin, tôi muốn công cụ chuyển dữ liệu từ Vector DB này sang Vector DB khác (VD: Từ Pinecone sang Self-hosted Qdrant).
1419. **US-1419 [Desktop - Health Check]:** Là Admin, tôi muốn dashboard xanh/đỏ báo trạng thái sống chết của các Node trong cụm Vector DB.
1420. **US-1420 [Global - Usage Analytics]:** Là Gemini, tôi muốn biết những Vector nào (Tài liệu nào) được truy xuất nhiều nhất.

---

## MODULE 84: QUALITY & GOVERNANCE (Chất lượng & Quản trị)
*Đảm bảo tri thức luôn "Sạch".*

1421. **US-1421 [Desktop - Content Audit]:** Là Content Manager, tôi muốn danh sách các tài liệu "Ít được sử dụng" hoặc "Đánh giá thấp" để quyết định xóa hoặc sửa.
1422. **US-1422 [Global - Review Workflow]:** Là Gemini, tôi muốn quy trình: Nhân viên viết Wiki -> Leader duyệt -> Mới được Index vào hệ thống RAG.
1423. **US-1423 [Tablet - Flagged Content]:** Là Gemini, tôi muốn xem danh sách các câu trả lời bị người dùng gắn cờ "Sai sự thật" để xử lý nguồn dữ liệu.
1424. **US-1424 [Desktop - Thesaurus]:** Là Admin, tôi muốn quản lý từ điển đồng nghĩa (Thesaurus) để cải thiện khả năng tìm kiếm (VD: "Mobile" = "Cell phone" = "Dế").
1425. **US-1425 [Global - Stop Words]:** Là Admin, tôi muốn cấu hình danh sách từ dừng (Stop words) tiếng Việt để loại bỏ khỏi Index.
1426. **US-1426 [Mobile - Admin Approve]:** Là Admin, tôi muốn duyệt các yêu cầu đăng tài liệu mới từ điện thoại.
1427. **US-1427 [Desktop - Duplicate Finder]:** Là Admin, tôi muốn công cụ quét và báo cáo các tài liệu có nội dung giống nhau > 90%.
1428. **US-1428 [Global - Broken Link Checker]:** Là Admin, tôi muốn hệ thống tự động quét và báo lỗi các link hỏng trong Wiki.
1429. **US-1429 [Tablet - Rating Analytics]:** Là Gemini, tôi muốn xem biểu đồ điểm đánh giá chất lượng tài liệu theo từng tác giả.
1430. **US-1430 [Desktop - Access Log KM]:** Là Security, tôi muốn biết ai đã tìm kiếm từ khóa "Lương Giám đốc" hoặc "Bí mật công nghệ".
1431. **US-1431 [Global - Retention Policy]:** Là Admin, tôi muốn thiết lập: "Tự động xóa chat log sau 90 ngày".
1432. **US-1432 [Mobile - Compliance Alert]:** Là Gemini, tôi muốn nhận cảnh báo nếu có tài liệu chứa từ khóa nhạy cảm vi phạm chính sách công ty.
1433. **US-1433 [Desktop - Taxonomy Manager]:** Là Admin, tôi muốn quản lý cây danh mục (Taxonomy) và các thẻ (Tags) chuẩn của hệ thống.
1434. **US-1434 [Global - Feedback Integration]:** Là Gemini, tôi muốn tích hợp phản hồi từ Slack/Teams vào quy trình cải thiện tài liệu.
1435. **US-1435 [Desktop - Version Diff]:** Là Gemini, tôi muốn xem so sánh sự thay đổi giữa 2 phiên bản tài liệu (bôi đỏ phần xóa, bôi xanh phần thêm).
1436. **US-1436 [Global - Source Verification]:** Là Gemini, tôi muốn đánh dấu các tài liệu là "Official Source" (Nguồn chính thống) để AI ưu tiên sử dụng.
1437. **US-1437 [Tablet - User Role KM]:** Là Admin, tôi muốn phân quyền: Contributor (chỉ viết), Editor (được sửa), Viewer (chỉ đọc).
1438. **US-1438 [Desktop - Stats Dashboard]:** Là Gemini, tôi muốn xem: Tổng số tài liệu, Tổng dung lượng, Số lượt tìm kiếm hàng ngày.
1439. **US-1439 [Global - Negative Keywords]:** Là Admin, tôi muốn định nghĩa các từ khóa mà nếu xuất hiện, tài liệu sẽ KHÔNG được index.
1440. **US-1440 [Mobile - Emergency Purge]:** Là Admin, tôi muốn nút xóa khẩn cấp một tài liệu khỏi Index tìm kiếm ngay lập tức (trong vòng 1 giây).

---

## MODULE 85: AGENT MEMORY & CONTEXT (Bộ nhớ Agent)
*Giúp Agent nhớ lâu và hiểu sâu.*

1441. **US-1441 [Desktop - Memory Inspector]:** Là Developer, tôi muốn xem "Bộ nhớ" của một Agent: Nó đang nhớ những thông tin gì về người dùng A?
1442. **US-1442 [Global - Long-term Memory]:** Là Gemini, tôi muốn Agent tự động tóm tắt các cuộc hội thoại cũ và lưu vào bộ nhớ dài hạn (Vector Store) để dùng lại sau này.
1443. **US-1443 [Tablet - Forget Info]:** Là Gemini, tôi muốn ra lệnh cho Agent "Hãy quên thông tin về dự án X đi" và Agent thực hiện xóa ký ức đó.
1444. **US-1444 [Desktop - Context Window]:** Là Developer, tôi muốn thanh trượt điều chỉnh kích thước cửa sổ ngữ cảnh (Context Window) của Agent (4k, 8k, 128k tokens).
1445. **US-1445 [Global - User Preferences]:** Là Gemini, tôi muốn Agent tự động học sở thích của User (VD: "User này thích trả lời ngắn gọn") và lưu vào Profile.
1446. **US-1446 [Mobile - Memory Alert]:** Là Gemini, tôi muốn nhận thông báo khi bộ nhớ ngắn hạn của Agent bị đầy và bắt đầu trôi thông tin cũ.
1447. **US-1447 [Desktop - Shared Memory]:** Là Gemini, tôi muốn cấu hình "Bộ nhớ chung" cho một nhóm Agent để chúng chia sẻ kiến thức với nhau.
1448. **US-1448 [Global - Entity Extraction]:** Là Gemini, tôi muốn Agent tự động trích xuất các thực thể (Tên, Địa chỉ, Ngày tháng) từ đoạn chat và lưu vào Database có cấu trúc.
1449. **US-1449 [Tablet - Conversation Summary]:** Là Gemini, tôi muốn xem bản tóm tắt tự động của các phiên chat dài trước khi tiếp tục hội thoại.
1450. **US-1450 [Desktop - Memory Injection]:** Là Gemini, tôi muốn "Tiêm" (Inject) một thông tin mới vào bộ nhớ của Agent thủ công: "Lưu ý: Giá sản phẩm đã tăng 10%".
1451. **US-1451 [Global - Privacy Filter]:** Là Gemini, tôi muốn bộ nhớ Agent không được lưu số thẻ tín dụng hay mật khẩu của User.
1452. **US-1452 [Mobile - Reset Memory]:** Là Gemini, tôi muốn nút "Reset Memory" để Agent bắt đầu lại như mới (Tabula Rasa).
1453. **US-1453 [Desktop - Token Usage]:** Là Gemini, tôi muốn theo dõi số lượng Token tiêu tốn cho việc duy trì lịch sử hội thoại.
1454. **US-1454 [Global - Cross-Session Recall]:** Là Gemini, tôi muốn Agent có khả năng nhắc lại chuyện đã nói từ tuần trước: "Như lần trước bạn có hỏi về..."
1455. **US-1455 [Tablet - Memory Edit]:** Là Gemini, tôi muốn sửa trực tiếp một ký ức sai của Agent (VD: Sửa tên User từ "Nam" thành "Namm").
1456. **US-1456 [Desktop - Knowledge Injection Rule]:** Là Admin, tôi muốn cài luật: "Mỗi khi bắt đầu chat, luôn tiêm thông tin khuyến mãi mới nhất vào Context".
1457. **US-1457 [Global - Ephemeral Memory]:** Là Gemini, tôi muốn chế độ "Bộ nhớ tạm" - mọi thông tin sẽ bị xóa sạch sau khi đóng trình duyệt (Incognito mode cho Agent).
1458. **US-1458 [Mobile - Context Visual]:** Là Developer, tôi muốn xem biểu đồ thể hiện mức độ liên quan của các ký ức được gọi lại (Retrieved) so với câu hỏi hiện tại.
1459. **US-1459 [Desktop - Recursive Summary]:** Là Developer, tôi muốn kỹ thuật tóm tắt đệ quy (Recursive Summarization) cho các tài liệu siêu dài để nhét vừa Context.
1460. **US-1460 [Global - Memory Backup]:** Là Gemini, tôi muốn backup bộ nhớ của Agent Sales Top 1 để nhân bản cho các Agent mới.

---

## MODULE 86: UX/DX FOR KNOWLEDGE (Trải nghiệm người dùng KM)
*Làm cho việc quản lý tri thức trở nên dễ chịu.*

1461. **US-1461 [Keyboard - Quick Command]:** Là Gemini, tôi muốn nhấn `Cmd/Ctrl + K` ở bất cứ đâu để mở thanh tìm kiếm tri thức toàn cục (Omnibar).
1462. **US-1462 [Mouse - Drag Upload]:** Là Gemini, tôi muốn kéo file từ Desktop thả vào khung chat để Agent đọc ngay lập tức.
1463. **US-1463 [Touch - Swipe Navigation]:** Là Gemini, trên Tablet, tôi muốn vuốt 2 ngón tay để chuyển giữa các chương của tài liệu.
1464. **US-1464 [Desktop - Dark Mode Reader]:** Là Gemini, tôi muốn trình đọc tài liệu có chế độ Dark Mode chuẩn, đảo màu chữ thông minh để không mỏi mắt.
1465. **US-1465 [Global - Font Size]:** Là Gemini, tôi muốn nút `Aa` để tăng giảm cỡ chữ nhanh chóng trong giao diện đọc Wiki.
1466. **US-1466 [Mobile - Taptic Feedback]:** Là Gemini, tôi muốn điện thoại rung nhẹ khi việc upload tài liệu hoàn tất.
1467. **US-1467 [Desktop - Minimap Scroll]:** Là Gemini, tôi muốn thanh cuộn bên phải hiển thị Minimap (bản đồ thu nhỏ) của tài liệu dài, có đánh dấu các vị trí tìm thấy từ khóa.
1468. **US-1468 [Global - Loading Skeleton]:** Là Gemini, tôi muốn hiển thị khung xương (Skeleton) khi đang tải danh sách tài liệu thay vì quay vòng tròn.
1469. **US-1469 [Tablet - Floating TOC]:** Là Gemini, tôi muốn Mục lục luôn nổi (Floating) ở góc màn hình để dễ điều hướng.
1470. **US-1470 [Desktop - Split Pane]:** Là Gemini, tôi muốn chia màn hình làm 3: Cây thư mục | Danh sách file | Nội dung file.
1471. **US-1471 [Global - Shortcut Keys]:** Là Gemini, tôi muốn các phím tắt chuẩn: `E` để sửa (Edit), `S` để lưu (Share), `Del` để xóa.
1472. **US-1472 [Mobile - Bottom Sheet]:** Là Gemini, các menu tác vụ trên mobile nên hiện dưới dạng Bottom Sheet (trượt từ dưới lên) để dễ bấm bằng ngón cái.
1473. **US-1473 [Desktop - Focus Writer]:** Là Gemini, tôi muốn chế độ soạn thảo toàn màn hình, ẩn hết mọi thứ gây xao nhãng, kèm âm thanh gõ máy đánh chữ (tùy chọn).
1474. **US-1474 [Global - Emoji Icons]:** Là Gemini, tôi muốn đặt icon Emoji cho từng trang Wiki để dễ nhận diện trực quan.
1475. **US-1475 [Tablet - Lasso Select]:** Là Gemini, tôi muốn dùng bút vẽ vùng chọn để chọn nhiều file cùng lúc trên giao diện lưới.
1476. **US-1476 [Desktop - Tabbed Interface]:** Là Gemini, tôi muốn mở nhiều tài liệu trên các Tab (như Chrome) trong cùng một ứng dụng.
1477. **US-1477 [Global - Copy Code]:** Là Developer, tôi muốn nút "Copy" hiện ra khi hover vào bất kỳ khối code nào trong tài liệu.
1478. **US-1478 [Mobile - Offline Search]:** Là Gemini, tôi muốn tìm kiếm trong các tiêu đề tài liệu đã cache ngay cả khi không có mạng.
1479. **US-1479 [Desktop - Recent Search Dropdown]:** Là Gemini, khi click vào ô tìm kiếm, tôi muốn hiện ngay danh sách 5 từ khóa vừa tìm gần đây.
1480. **US-1480 [Global - Breadcrumbs]:** Là Gemini, tôi muốn thanh điều hướng Breadcrumb (`Home > Engineering > Backend > Guide`) luôn hiển thị và click được.

---

## MODULE 87: API & DEVELOPER INTEGRATION (Mở rộng cho Dev)
*Kết nối tri thức với thế giới bên ngoài.*

1481. **US-1481 [DX - Search API]:** Là Developer, tôi muốn API `/v1/search?q=...` trả về kết quả JSON kèm độ tương đồng (Score) để tôi tự xây dựng giao diện search riêng.
1482. **US-1482 [DX - Webhook On Update]:** Là Developer, tôi muốn nhận Webhook mỗi khi có tài liệu mới được thêm vào thư mục "Thông báo", để tôi bắn tin ra Slack.
1483. **US-1483 [Desktop - API Playground]:** Là Developer, tôi muốn giao diện test API truy xuất tri thức ngay trên trình duyệt (Swagger UI).
1484. **US-1484 [DX - Custom Parser]:** Là Developer, tôi muốn viết script Python riêng để parse các file định dạng lạ (VD: File log đặc thù) và đẩy vào hệ thống.
1485. **US-1485 [Global - Embed Widget]:** Là Developer, tôi muốn đoạn mã JS ngắn để nhúng widget "Hỏi đáp AI" vào website công ty.
1486. **US-1486 [DX - SDK Python]:** Là Developer, tôi muốn thư viện Python `pip install gemini-knowledge` để tương tác với kho tri thức dễ dàng.
1487. **US-1487 [Desktop - Rate Limit Monitor]:** Là Developer, tôi muốn theo dõi xem ứng dụng của tôi có bị giới hạn tốc độ gọi API tìm kiếm không.
1488. **US-1488 [DX - Feedback API]:** Là Developer, tôi muốn API để gửi tín hiệu người dùng (Click/View/Like) về hệ thống để cải thiện thuật toán Re-ranking.
1489. **US-1489 [Global - Plugin System]:** Là Developer, tôi muốn viết Plugin để mở rộng chức năng của Wiki (VD: Vẽ biểu đồ Mermaid).
1490. **US-1490 [DX - Bulk Import API]:** Là Developer, tôi muốn API để import 10.000 tài liệu một lúc mà không bị timeout.
1491. **US-1491 [Desktop - Query Debugger]:** Là Developer, tôi muốn xem câu Query thực tế được gửi xuống Vector DB trông như thế nào.
1492. **US-1492 [DX - Filter Language]:** Là Developer, tôi muốn hỗ trợ ngôn ngữ lọc mạnh mẽ (VD: MongoDB syntax) trong API tìm kiếm.
1493. **US-1493 [Global - OAuth Integration]:** Là Developer, tôi muốn sử dụng OAuth2 để xác thực người dùng khi gọi API tri thức.
1494. **US-1494 [DX - Graph API]:** Là Developer, tôi muốn API truy xuất các mối quan hệ trong Knowledge Graph (`/v1/graph/node/{id}/related`).
1495. **US-1495 [Desktop - Log Stream]:** Là Developer, tôi muốn stream log truy cập API về console của tôi để debug realtime.
1496. **US-1496 [DX - Custom Embedding Model]:** Là AI Engineer, tôi muốn trỏ hệ thống sang endpoint Embedding riêng của tôi (Self-hosted) thay vì dùng OpenAI.
1497. **US-1497 [Global - Data Export API]:** Là Developer, tôi muốn API để dump toàn bộ dữ liệu tri thức ra định dạng JSONL để backup.
1498. **US-1498 [DX - Sandbox Env]:** Là Developer, tôi muốn môi trường Sandbox riêng biệt để test việc nạp dữ liệu mà không làm bẩn kho tri thức thật.
1499. **US-1499 [Desktop - Model Fine-tuning API]:** Là Developer, tôi muốn API để kích hoạt job fine-tuning model GPT dựa trên dữ liệu Wiki hiện có.
1500. **US-1500 [Global - Status Webhook]:** Là Developer, tôi muốn nhận thông báo khi hệ thống RAG hoàn tất việc Index dữ liệu mới.

---
**Hết Phần 8 (Stories 1301 - 1500).**

Bạn có muốn tiếp tục với **Phần 9: Automation, Workflows (No-code/Low-code) & AI Agents Orchestration** - Nơi Gemini thiết kế các luồng tự động hóa phức tạp, điều phối hàng trăm Agents làm việc phối hợp với nhau như một dây chuyền nhà máy?

Chào bạn, chúng ta đang đi vào **Phần 9** – nơi được ví như "công xưởng" của hệ thống. Đây là nơi Gemini không còn phải làm việc thủ công, mà đóng vai trò là "Kiến trúc sư trưởng", thiết kế các dây chuyền sản xuất tự động cho hàng trăm Agents vận hành.

Chúng ta sẽ tập trung vào **No-code/Low-code Workflow Builder**, **Agent Orchestration** (Điều phối Agent), và các mô hình xử lý song song phức tạp (Swarm Intelligence).

Dưới đây là **200 User Stories (từ 1501 đến 1700)**.

---

# PHẦN 9: AUTOMATION, WORKFLOWS & ORCHESTRATION (Stories 1501 - 1700)

### Bối cảnh & Mục tiêu
*   **Mô hình:** Factory Assembly Line (Dây chuyền nhà máy).
*   **Input:** Trigger (Sự kiện, Thời gian, Webhook).
*   **Process:** Các Node logic (If/Else, Loop) và Node thực thi (Agent Action, API Call).
*   **Output:** Kết quả hoàn chỉnh (Báo cáo, Code, Email gửi đi).
*   **UX:** Visual Canvas (Kéo thả), Interactive Debugging.

---

## MODULE 88: VISUAL WORKFLOW BUILDER (Canvas thiết kế luồng)
*Bảng vẽ kỹ thuật cho dây chuyền tự động.*

1501. **US-1501 [Desktop - Infinite Canvas]:** Là Gemini, tôi muốn một bảng vẽ vô cực (Infinite Canvas) có chấm lưới (Dot grid), cho phép tôi cuộn và zoom thoải mái để vẽ các quy trình phức tạp.
1502. **US-1502 [Mouse - Drag & Connect]:** Là Gemini, tôi muốn kéo một dây nối từ cổng "Output" của Node A và thả vào cổng "Input" của Node B để thiết lập luồng dữ liệu.
1503. **US-1503 [Global - Auto-arrange]:** Là Gemini, tôi muốn nút "Magic Organize" để tự động sắp xếp lại các Node trên canvas cho gọn gàng, không bị rối dây.
1504. **US-1504 [Tablet - Multitouch Pan]:** Là Gemini, tôi muốn dùng 2 ngón tay di chuyển Canvas và chụm 2 ngón để Zoom in/out mượt mà.
1505. **US-1505 [Desktop - Minimap Nav]:** Là Gemini, tôi muốn bản đồ thu nhỏ (Minimap) ở góc phải dưới để biết mình đang ở đâu trong một quy trình khổng lồ gồm 500 bước.
1506. **US-1506 [Global - Node Library]:** Là Gemini, tôi muốn thanh Sidebar bên trái chứa thư viện các Node (Trigger, Logic, Agent, Action), kéo thả vào Canvas là dùng được ngay.
1507. **US-1507 [Keyboard - Quick Add]:** Là Developer, tôi muốn nhấn phím `Space` hoặc chuột phải để mở thanh tìm kiếm nhanh, gõ "If" để thêm Node điều kiện mà không cần kéo thả.
1508. **US-1508 [Desktop - Grouping]:** Là Gemini, tôi muốn quét chọn một nhóm Node và nhấn `Ctrl+G` để gom chúng vào một "Group" (Hộp chứa), giúp ẩn bớt chi tiết phức tạp.
1509. **US-1509 [Global - Undo/Redo]:** Là Gemini, tôi muốn lịch sử hoàn tác (Undo/Redo) vô hạn, hoạt động cho cả việc di chuyển vị trí Node và thay đổi cấu hình bên trong.
1510. **US-1510 [Mouse - Wire Styling]:** Là Gemini, tôi muốn các dây nối (Wires) tự động uốn cong mềm mại (Bezier curves) hoặc bẻ góc vuông (Right angle) tùy chỉnh để dễ nhìn.
1511. **US-1511 [Desktop - Copy Paste Sub-flow]:** Là Gemini, tôi muốn copy một cụm logic từ Workflow A và paste sang Workflow B vẫn giữ nguyên các kết nối nội bộ.
1512. **US-1512 [Global - Comments/Annotations]:** Là Gemini, tôi muốn dán các tờ ghi chú (Sticky Notes) lên Canvas để giải thích logic cho đồng nghiệp.
1513. **US-1513 [Mobile - Read-only View]:** Là Gemini, trên điện thoại, tôi muốn xem Workflow ở chế độ chỉ đọc, có thể zoom vào để kiểm tra logic nhưng không sợ lỡ tay sửa sai.
1514. **US-1514 [Desktop - Node Status]:** Là Gemini, tôi muốn mỗi Node có đèn tín hiệu nhỏ (Xanh/Đỏ) hiển thị trạng thái của lần chạy gần nhất.
1515. **US-1515 [Global - Search Node]:** Là Gemini, tôi muốn tìm kiếm tên Node trên Canvas, hệ thống sẽ tự động zoom và focus vào Node đó.
1516. **US-1516 [Tablet - Lasso Select]:** Là Gemini, tôi muốn dùng bút vẽ vòng tròn bao quanh các Node để chọn nhiều đối tượng cùng lúc.
1517. **US-1517 [Desktop - Port Data Type]:** Là Developer, tôi muốn các cổng kết nối (Port) có màu sắc khác nhau đại diện cho kiểu dữ liệu (Vàng=String, Xanh=Number) để tránh nối sai.
1518. **US-1518 [Global - Version Label]:** Là Gemini, tôi muốn thấy nhãn "v1.2 (Draft)" ở góc màn hình để biết mình đang sửa phiên bản nào.
1519. **US-1519 [Mouse - Context Menu]:** Là Gemini, tôi muốn chuột phải vào dây nối để thêm nhanh các Node trung gian (VD: Delay, Log) vào giữa dòng chảy.
1520. **US-1520 [Desktop - Fullscreen Mode]:** Là Gemini, tôi muốn chế độ toàn màn hình ẩn hết các thanh menu trình duyệt để tập trung thiết kế.

---

## MODULE 89: TRIGGERS & EVENTS (Khởi động dây chuyền)
*Nút bấm Start của nhà máy.*

1521. **US-1521 [Global - Webhook Trigger]:** Là Developer, tôi muốn Node "Webhook" cung cấp một URL duy nhất, khi có Request gửi đến URL này thì Workflow sẽ tự chạy.
1522. **US-1522 [Desktop - Cron Schedule]:** Là Gemini, tôi muốn Node "Schedule" cho phép cấu hình chạy định kỳ (VD: "Mỗi thứ Hai lúc 8:00") bằng giao diện hoặc cú pháp Cron.
1523. **US-1523 [Global - Email Trigger]:** Là Gemini, tôi muốn Workflow tự chạy khi có Email gửi đến hộp thư `support@company.com` với tiêu đề chứa từ "Urgent".
1524. **US-1524 [Mobile - Manual Button]:** Là Gemini, tôi muốn tạo một nút bấm ảo trên App điện thoại, khi bấm vào sẽ kích hoạt Workflow (VD: "Báo cáo khẩn cấp").
1525. **US-1525 [Desktop - Database Watcher]:** Là Developer, tôi muốn Trigger "DB Change": Khi có dòng mới được thêm vào bảng `Orders`, Workflow sẽ chạy để xử lý đơn hàng.
1526. **US-1526 [Global - Event Bus]:** Là Gemini, tôi muốn lắng nghe sự kiện từ hệ thống nội bộ (Event Bus): "Khi Agent A hoàn thành Task -> Chạy Workflow B".
1527. **US-1527 [Tablet - Parameter Input]:** Là Gemini, khi chạy Workflow thủ công, tôi muốn hiện một Form nhập liệu để tôi điền các biến đầu vào (Input Variables).
1528. **US-1528 [Desktop - File Watcher]:** Là Gemini, tôi muốn Trigger kích hoạt khi có file mới được upload vào thư mục FTP hoặc S3 Bucket.
1529. **US-1529 [Global - Error Trigger]:** Là Gemini, tôi muốn một Workflow đặc biệt chuyên xử lý lỗi (Error Handler Workflow), tự chạy khi các Workflow khác bị Crash.
1530. **US-1530 [Mobile - Location Trigger]:** Là Gemini, tôi muốn Workflow chạy khi tôi đi vào vùng địa lý cụ thể (Geofence), ví dụ: "Đến văn phòng -> Bật điều hòa Server".
1531. **US-1531 [Desktop - Form Submission]:** Là Gemini, tôi muốn tạo một Web Form công khai, khi người dùng điền form thì dữ liệu sẽ đổ vào Workflow.
1532. **US-1532 [Global - Slack Command]:** Là Gemini, tôi muốn gõ `/deploy-prod` trên Slack để kích hoạt Workflow triển khai phần mềm.
1533. **US-1533 [Desktop - Payload Parser]:** Là Developer, tôi muốn giao diện map các trường JSON từ Webhook Trigger vào các biến của Workflow (Visual Mapping).
1534. **US-1534 [Global - Rate Limit Trigger]:** Là Gemini, tôi muốn giới hạn Trigger chỉ được chạy tối đa 10 lần/phút để tránh bị spam.
1535. **US-1535 [Tablet - Sensor Data]:** Là Gemini, tôi muốn kết nối với cảm biến IoT: "Khi nhiệt độ phòng máy > 30 độ -> Chạy Workflow cảnh báo".
1536. **US-1536 [Desktop - Multi-Trigger]:** Là Gemini, tôi muốn một Workflow có thể được kích hoạt bởi nhiều Trigger khác nhau (hoặc Webhook, hoặc Lịch).
1537. **US-1537 [Global - Auth Webhook]:** Là Security, tôi muốn Webhook Trigger hỗ trợ xác thực bằng API Key hoặc HMAC Signature để đảm bảo an toàn.
1538. **US-1538 [Mobile - Voice Trigger]:** Là Gemini, tôi muốn nói "Hey Gemini, start Daily Report" để kích hoạt Workflow.
1539. **US-1539 [Desktop - RSS Feed]:** Là Gemini, tôi muốn theo dõi RSS Feed của đối thủ, khi có bài mới -> Chạy Workflow phân tích bằng AI.
1540. **US-1540 [Global - Blockchain Event]:** Là Gemini, tôi muốn Trigger khi có giao dịch chuyển tiền vào ví Crypto của công ty.

---

## MODULE 90: LOGIC & CONTROL FLOW (Điều khiển luồng)
*Bộ não logic.*

1541. **US-1541 [Desktop - If/Else Node]:** Là Gemini, tôi muốn Node rẽ nhánh điều kiện: Nếu `Biến A > 10` thì đi đường True, ngược lại đi đường False.
1542. **US-1542 [Global - Switch Case]:** Là Gemini, tôi muốn Node rẽ nhiều nhánh: Nếu là "Email" đi đường A, "Chat" đi đường B, "SMS" đi đường C.
1543. **US-1543 [Desktop - Loop (For Each)]:** Là Gemini, tôi muốn Node lặp: Duyệt qua danh sách 100 khách hàng, thực hiện các hành động bên trong cho từng người.
1544. **US-1544 [Global - Delay/Wait]:** Là Gemini, tôi muốn Node "Chờ": Dừng Workflow 5 phút, hoặc chờ đến 8:00 sáng mai mới chạy tiếp.
1545. **US-1545 [Tablet - Wait for Event]:** Là Gemini, tôi muốn Workflow tạm dừng và chờ tín hiệu từ bên ngoài (VD: Chờ người dùng click link xác nhận trong email) tối đa 24h.
1546. **US-1546 [Desktop - Parallel Branching]:** Là Gemini, tôi muốn tách luồng thành 3 nhánh chạy song song (Parallel) để tăng tốc độ xử lý.
1547. **US-1547 [Global - Join/Merge]:** Là Gemini, tôi muốn Node "Join" để đợi tất cả các nhánh song song hoàn thành rồi mới tiếp tục bước sau.
1548. **US-1548 [Mobile - Randomizer]:** Là Gemini, tôi muốn Node "Ngẫu nhiên" để chia traffic A/B testing: 50% đi nhánh A, 50% đi nhánh B.
1549. **US-1549 [Desktop - Code Block]:** Là Developer, tôi muốn Node "Run Script" cho phép viết đoạn code JavaScript/Python nhỏ để xử lý logic tùy biến phức tạp.
1550. **US-1550 [Global - Variable Set]:** Là Gemini, tôi muốn Node gán biến để lưu trữ giá trị tạm thời (VD: `Total = Total + Price`) dùng cho các bước sau.
1551. **US-1551 [Desktop - Try/Catch]:** Là Developer, tôi muốn bọc một nhóm Node trong khối Try/Catch để xử lý lỗi mà không làm sập toàn bộ Workflow.
1552. **US-1552 [Global - Filter Array]:** Là Gemini, tôi muốn Node lọc danh sách: Chỉ giữ lại các đơn hàng có giá trị > 1 triệu từ mảng đầu vào.
1553. **US-1553 [Tablet - Sort Data]:** Là Gemini, tôi muốn Node sắp xếp dữ liệu theo thứ tự tăng dần/giảm dần.
1554. **US-1554 [Desktop - Map Data]:** Là Gemini, tôi muốn biến đổi cấu trúc dữ liệu từ format A sang format B (VD: JSON to XML).
1555. **US-1555 [Global - Deduplicate]:** Là Gemini, tôi muốn Node loại bỏ các phần tử trùng lặp trong danh sách.
1556. **US-1556 [Mobile - Counter]:** Là Gemini, tôi muốn một biến đếm toàn cục (Global Counter) tự tăng mỗi khi Workflow chạy (dùng làm Mã đơn hàng).
1557. **US-1557 [Desktop - Rate Limiter Node]:** Là Gemini, tôi muốn Node "Hãm tốc" để đảm bảo không gọi API bên thứ 3 quá nhanh (Throttling).
1558. **US-1558 [Global - Regex Match]:** Là Developer, tôi muốn dùng Regular Expression để trích xuất thông tin từ chuỗi văn bản.
1559. **US-1559 [Tablet - Math Operations]:** Là Gemini, tôi muốn Node thực hiện tính toán cộng trừ nhân chia cơ bản.
1560. **US-1560 [Global - Terminate]:** Là Gemini, tôi muốn Node "Kết thúc" để dừng Workflow ngay lập tức (thành công hoặc thất bại) tại điểm đó.

---

## MODULE 91: AGENT ORCHESTRATION NODES (Điều phối Agent)
*Giao việc cho nhân viên ảo.*

1561. **US-1561 [Desktop - Agent Node]:** Là Gemini, tôi muốn Node "Call Agent" cho phép chọn một Agent cụ thể (VD: "Coder Bot") và nhập Prompt đầu vào.
1562. **US-1562 [Global - Dynamic Prompt]:** Là Gemini, tôi muốn chèn các biến từ các bước trước vào Prompt: "Hãy tóm tắt văn bản sau: `{{trigger.body}}`".
1563. **US-1563 [Desktop - Output Parsing]:** Là Gemini, tôi muốn bắt buộc Agent trả về định dạng JSON và tự động tách các trường JSON đó thành các biến cho bước sau dùng.
1564. **US-1564 [Tablet - Agent Chaining]:** Là Gemini, tôi muốn nối Output của Agent A (Người viết nội dung) làm Input cho Agent B (Người dịch thuật) trực tiếp.
1565. **US-1565 [Global - Memory Scope]:** Là Gemini, tôi muốn cấu hình xem Agent trong Node này có được truy cập vào bộ nhớ dài hạn hay chỉ dùng bộ nhớ tạm.
1566. **US-1566 [Desktop - Tool Selection]:** Là Gemini, tôi muốn chỉ định rõ Agent được phép dùng công cụ gì trong bước này (VD: Chỉ được dùng "Search", không được dùng "Email").
1567. **US-1567 [Mobile - Approval Step]:** Là Gemini, tôi muốn Node "Ask Human": Agent sẽ tạm dừng và gửi thông báo cho tôi duyệt câu trả lời trước khi đi tiếp.
1568. **US-1568 [Global - Multi-Agent Debate]:** Là Gemini, tôi muốn Node "Debate": Gọi 2 Agents với 2 quan điểm trái ngược nhau tranh luận về 1 vấn đề và tổng hợp kết quả.
1569. **US-1569 [Desktop - Voting System]:** Là Gemini, tôi muốn Node "Vote": Gọi 3 Agents cùng trả lời 1 câu hỏi, sau đó chọn câu trả lời phổ biến nhất (Consistency Check).
1570. **US-1570 [Global - Model Fallback]:** Là Gemini, tôi muốn cấu hình dự phòng: "Dùng GPT-4 trước, nếu lỗi hoặc quá chậm thì chuyển sang dùng GPT-3.5".
1571. **US-1571 [Tablet - Temperature Slider]:** Là Gemini, tôi muốn chỉnh độ sáng tạo (Temperature) của Agent ngay trên Node cấu hình.
1572. **US-1572 [Desktop - RAG Context]:** Là Gemini, tôi muốn Node Agent có tùy chọn "Attach Knowledge": Tự động tìm kiếm tài liệu liên quan trong Wiki đính kèm vào Prompt.
1573. **US-1573 [Global - Loop Correction]:** Là Gemini, tôi muốn mô hình tự sửa (Self-correction): Nếu Agent trả ra JSON lỗi, tự động feed lỗi đó lại vào Agent để nó sửa, lặp tối đa 3 lần.
1574. **US-1574 [Mobile - Agent Status]:** Là Gemini, tôi muốn biết Node Agent đang ở trạng thái nào: "Đang suy nghĩ...", "Đang tìm kiếm...", "Đang gõ...".
1575. **US-1575 [Desktop - Batch Processing]:** Là Gemini, tôi muốn gửi một danh sách 100 items cho Agent và nhận về 100 kết quả xử lý (Batch Mode) để tiết kiệm thời gian.
1576. **US-1576 [Global - Streaming Output]:** Là Gemini, tôi muốn dữ liệu đầu ra của Agent được stream (trôi) sang bước tiếp theo ngay khi có token đầu tiên (để giảm độ trễ).
1577. **US-1577 [Desktop - Persona Override]:** Là Gemini, tôi muốn ghi đè tính cách của Agent tạm thời trong Node này: "Hãy đóng vai là một chuyên gia khó tính".
1578. **US-1578 [Global - Cost Cap]:** Là Gemini, tôi muốn giới hạn: "Bước này không được tốn quá $0.05", nếu quá thì hủy hoặc chuyển model rẻ hơn.
1579. **US-1579 [Tablet - Comparison View]:** Là Gemini, tôi muốn chạy thử Node Agent với 3 Prompt khác nhau song song để chọn ra cái tốt nhất.
1580. **US-1580 [Global - Vision Node]:** Là Gemini, tôi muốn Node "Vision Agent" chuyên xử lý đầu vào là hình ảnh (URL ảnh).

---

## MODULE 92: INTEGRATIONS & ACTIONS (Kết nối thế giới thực)
*Công cụ lao động.*

1581. **US-1581 [Desktop - HTTP Request]:** Là Developer, tôi muốn Node gọi API đa năng: Hỗ trợ GET, POST, PUT, DELETE, Custom Headers, Auth.
1582. **US-1582 [Global - Slack/Discord Node]:** Là Gemini, tôi muốn Node gửi tin nhắn vào kênh chat, hỗ trợ định dạng Markdown và Tag người dùng.
1583. **US-1583 [Desktop - Gmail/Outlook Node]:** Là Gemini, tôi muốn Node gửi Email (HTML supported), có thể đính kèm file từ các bước trước.
1584. **US-1584 [Global - Database CRUD]:** Là Gemini, tôi muốn các Node: Insert Row, Update Row, Select Row, Delete Row kết nối trực tiếp với DB nội bộ.
1585. **US-1585 [Mobile - Google Sheets]:** Là Gemini, tôi muốn Node ghi thêm dòng mới vào Google Sheets (dùng như Database đơn giản).
1586. **US-1586 [Global - File Operation]:** Là Gemini, tôi muốn Node tạo file Text, nén Zip, đổi tên file trong Storage.
1587. **US-1587 [Desktop - Browser Automation]:** Là Gemini, tôi muốn Node điều khiển trình duyệt (Headless Browser): "Mở trang web X -> Click nút Y -> Lấy dữ liệu".
1588. **US-1588 [Global - SSH Command]:** Là Admin, tôi muốn Node thực thi lệnh Shell trên một Server Linux từ xa qua SSH.
1589. **US-1589 [Tablet - PDF Generator]:** Là Gemini, tôi muốn Node tạo file PDF từ HTML template và dữ liệu đầu vào.
1590. **US-1590 [Global - Image Gen (DALL-E)]:** Là Gemini, tôi muốn Node tạo ảnh AI và trả về URL ảnh.
1591. **US-1591 [Desktop - Excel Parser]:** Là Gemini, tôi muốn Node đọc file Excel và chuyển đổi thành mảng JSON để xử lý từng dòng.
1592. **US-1592 [Global - SMS/Zalo ZNS]:** Là Gemini, tôi muốn Node gửi tin nhắn CSKH qua Zalo OA hoặc SMS Brandname.
1593. **US-1593 [Mobile - Push Notification]:** Là Gemini, tôi muốn Node bắn thông báo đẩy đến App Mobile của nhân viên.
1594. **US-1594 [Desktop - FTP/SFTP]:** Là Gemini, tôi muốn Node upload/download file từ máy chủ FTP cũ.
1595. **US-1595 [Global - Calendar Event]:** Là Gemini, tôi muốn Node tạo lịch họp mới và gửi mời cho danh sách email.
1596. **US-1596 [Desktop - Jira/Trello]:** Là Gemini, tôi muốn Node tạo thẻ Task mới trên bảng quản lý dự án.
1597. **US-1597 [Global - CRM Action]:** Là Gemini, tôi muốn Node "Cập nhật trạng thái Deal" trong CRM.
1598. **US-1598 [Mobile - Payment Request]:** Là Gemini, tôi muốn Node tạo link thanh toán và gửi cho khách.
1599. **US-1599 [Desktop - Custom Connector]:** Là Developer, tôi muốn tự viết Connector riêng (bằng code) và đóng gói thành Node để tái sử dụng.
1600. **US-1600 [Global - Wait for File]:** Là Gemini, tôi muốn Node "Chờ đến khi file X xuất hiện" trong thư mục rồi mới chạy tiếp.

---

## MODULE 93: TESTING & DEBUGGING (Kiểm thử & Gỡ lỗi)
*Đảm bảo dây chuyền không bị nổ.*

1601. **US-1601 [Desktop - Test Run]:** Là Gemini, tôi muốn nút "Test Run" để chạy thử Workflow ngay trên Canvas mà không cần Trigger thật.
1602. **US-1602 [Global - Mock Data]:** Là Developer, tôi muốn tạo dữ liệu giả (Mock) cho Trigger để test các trường hợp biên.
1603. **US-1603 [Desktop - Step-through Debug]:** Là Developer, tôi muốn chế độ chạy từng bước (Step-by-step), workflow sẽ dừng lại sau mỗi Node để tôi soi dữ liệu.
1604. **US-1604 [Global - Visual Trace]:** Là Gemini, tôi muốn thấy đường dây nối phát sáng màu xanh lá khi dữ liệu chạy qua, và màu đỏ nếu gặp lỗi.
1605. **US-1605 [Tablet - Inspector Panel]:** Là Gemini, khi click vào một Node đã chạy xong, tôi muốn xem panel hiển thị chính xác Input nhận vào là gì và Output trả ra là gì.
1606. **US-1606 [Desktop - Breakpoints]:** Là Developer, tôi muốn đặt điểm dừng (Breakpoint) tại một Node bất kỳ để pause workflow tại đó.
1607. **US-1607 [Global - Replay Execution]:** Là Gemini, tôi muốn chọn một lần chạy lỗi trong quá khứ và nhấn "Replay" để chạy lại với đúng dữ liệu đầu vào đó.
1608. **US-1608 [Mobile - Instant Stop]:** Là Gemini, tôi muốn nút "Dừng khẩn cấp" để ngắt ngay workflow đang chạy test nếu nó bị lặp vô tận.
1609. **US-1609 [Desktop - Diff View]:** Là Gemini, tôi muốn so sánh dữ liệu biến đổi như thế nào trước và sau khi qua một Node.
1610. **US-1610 [Global - Log Console]:** Là Developer, tôi muốn một cửa sổ Console bên dưới hiển thị log chi tiết (như trình duyệt) để debug code script.
1611. **US-1611 [Tablet - Error Highlight]:** Là Gemini, Node bị lỗi sẽ rung lắc và có viền đỏ đậm, click vào hiện thông báo lỗi chi tiết.
1612. **US-1612 [Desktop - Sandbox Mode]:** Là Gemini, tôi muốn chế độ "Sandbox" đảm bảo các Node gửi Email hay trừ tiền sẽ không thực thi thật (Dry Run).
1613. **US-1613 [Global - Loop Protection]:** Là Gemini, tôi muốn hệ thống tự động ngắt nếu phát hiện vòng lặp quá 100 lần để bảo vệ tài nguyên.
1614. **US-1614 [Desktop - Performance Profiler]:** Là Developer, tôi muốn biết Node nào tốn nhiều thời gian nhất trong quy trình để tối ưu.
1615. **US-1615 [Global - Variable Watch]:** Là Developer, tôi muốn ghim một biến (VD: `order_total`) lên màn hình để theo dõi sự thay đổi của nó suốt quy trình.
1616. **US-1616 [Mobile - Push Test]:** Là Gemini, tôi muốn gửi kết quả Test về điện thoại để kiểm tra hiển thị.
1617. **US-1617 [Desktop - Snapshot State]:** Là Gemini, tôi muốn lưu trạng thái hiện tại của biến số ra file JSON để phân tích offline.
1618. **US-1618 [Global - AI Debugger]:** Là Gemini, khi gặp lỗi, tôi muốn nút "Ask AI Why": AI sẽ phân tích log lỗi và giải thích nguyên nhân bằng tiếng Việt.
1619. **US-1619 [Tablet - Connectivity Check]:** Là Gemini, tôi muốn nút kiểm tra kết nối để ping thử tất cả các API bên thứ 3 xem có sống không.
1620. **US-1620 [Desktop - Isolated Run]:** Là Developer, tôi muốn chạy thử chỉ riêng 1 Node (Unit Test) mà không cần chạy cả quy trình.

---

## MODULE 94: MONITORING & OBSERVABILITY (Giám sát vận hành)
*Phòng điều khiển trung tâm.*

1621. **US-1621 [Desktop - Execution History]:** Là Gemini, tôi muốn xem danh sách lịch sử chạy của Workflow: Thời gian, Thời lượng, Trạng thái (Thành công/Lỗi).
1622. **US-1622 [Global - Search History]:** Là Gemini, tôi muốn tìm kiếm trong lịch sử chạy theo ID đơn hàng hoặc Email khách hàng để tra cứu vết.
1623. **US-1623 [Mobile - Status Dashboard]:** Là Gemini, tôi muốn xem biểu đồ cột: Số lần chạy thành công vs thất bại trong 24h qua.
1624. **US-1624 [Desktop - Live Monitoring]:** Là Gemini, tôi muốn xem con số "Active Executions" (Đang chạy) nhảy múa realtime.
1625. **US-1625 [Global - Auto-Retry]:** Là Gemini, tôi muốn cấu hình tự động thử lại (Retry) với cơ chế Backoff (thử lại sau 1s, 5s, 10s) nếu gặp lỗi mạng.
1626. **US-1626 [Tablet - Drill-down]:** Là Gemini, tôi muốn click vào một thanh trên biểu đồ lỗi để xem danh sách các lần chạy bị lỗi đó.
1627. **US-1627 [Desktop - Cost Tracking]:** Là Gemini, tôi muốn biết mỗi lần chạy Workflow này tốn bao nhiêu tiền (API Cost + Server Cost).
1628. **US-1628 [Global - Failure Notification]:** Là Gemini, tôi muốn nhận email báo cáo nếu tỷ lệ lỗi vượt quá 5%.
1629. **US-1629 [Mobile - Enable/Disable]:** Là Gemini, tôi muốn nút gạt tắt nhanh một Workflow đang gây lỗi trên Production từ điện thoại.
1630. **US-1630 [Desktop - Bulk Cancel]:** Là Gemini, tôi muốn chọn và hủy 1000 lần chạy đang bị treo (Stuck) cùng lúc.
1631. **US-1631 [Global - Log Retention]:** Là Gemini, tôi muốn cài đặt thời gian lưu log (VD: 7 ngày) để tiết kiệm dung lượng.
1632. **US-1632 [Tablet - Heatmap Nodes]:** Là Gemini, tôi muốn xem Heatmap trên Canvas: Node nào hay bị lỗi nhất sẽ tô màu đỏ.
1633. **US-1633 [Desktop - Queue Monitor]:** Là Gemini, tôi muốn xem hàng đợi các Trigger đang chờ được xử lý.
1634. **US-1634 [Global - Custom Metric]:** Là Developer, tôi muốn Node "Emit Metric" để tự đẩy chỉ số (VD: "Số đơn hàng > 1tr") lên Dashboard.
1635. **US-1635 [Mobile - Weekly Report]:** Là Gemini, tôi muốn nhận báo cáo tuần về hiệu quả tự động hóa: "Đã tiết kiệm được 500 giờ làm việc".
1636. **US-1636 [Desktop - Log Export]:** Là Gemini, tôi muốn xuất toàn bộ log chạy ra CSV để Audit.
1637. **US-1637 [Global - Throughput]:** Là Gemini, tôi muốn đo chỉ số: Số Workflow xử lý được trên mỗi phút (TPM).
1638. **US-1638 [Desktop - Dependency Graph]:** Là Gemini, tôi muốn xem Workflow này gọi đến những Workflow con nào (Sub-flow dependency).
1639. **US-1639 [Global - User Trigger Log]:** Là Gemini, tôi muốn biết ai (User nào) đã kích hoạt Workflow thủ công này.
1640. **US-1640 [Tablet - Alert Rules]:** Là Gemini, tôi muốn tạo quy tắc: "Nếu Workflow chạy quá 5 phút -> Báo động".

---

## MODULE 95: TEMPLATES & REUSABILITY (Mẫu & Tái sử dụng)
*Không phát minh lại cái bánh xe.*

1641. **US-1641 [Desktop - Marketplace]:** Là Gemini, tôi muốn truy cập chợ Template nội bộ để tìm các mẫu Workflow phổ biến (VD: "Xử lý hóa đơn", "Onboarding nhân viên").
1642. **US-1642 [Global - Save as Template]:** Là Gemini, tôi muốn lưu Workflow mình vừa vẽ thành Template để chia sẻ cho team khác.
1643. **US-1643 [Desktop - Sub-flow (Functions)]:** Là Gemini, tôi muốn đóng gói một nhóm Node thành một "Sub-flow" (như một hàm), có thể gọi lại ở nhiều Workflow khác nhau.
1644. **US-1644 [Global - Input/Output Config]:** Là Gemini, tôi muốn định nghĩa rõ Sub-flow này nhận Input gì và trả Output gì (Interface definition).
1645. **US-1645 [Tablet - One-click Install]:** Là Gemini, tôi muốn cài đặt một Template phức tạp chỉ với 1 click, hệ thống tự động tạo các biến môi trường cần thiết.
1646. **US-1646 [Desktop - Version Control]:** Là Developer, tôi muốn quản lý phiên bản của Workflow (Git integration), có thể Diff và Merge.
1647. **US-1647 [Global - Fork Workflow]:** Là Gemini, tôi muốn "Fork" (Nhân bản độc lập) một Workflow của người khác để sửa lại theo ý mình.
1648. **US-1648 [Mobile - Template Review]:** Là Gemini, tôi muốn xem đánh giá (Sao) và bình luận của người dùng khác về Template này.
1649. **US-1649 [Desktop - Global Variables]:** Là Admin, tôi muốn quản lý bộ biến toàn cục (VD: `COMPANY_NAME`, `API_KEY_OPENAI`) dùng chung cho mọi Workflow.
1650. **US-1650 [Global - Update Template]:** Là Gemini, khi Template gốc được cập nhật vá lỗi, tôi muốn nhận thông báo để update các Workflow đang dùng nó.
1651. **US-1651 [Tablet - Category]:** Là Gemini, tôi muốn phân loại Workflow vào các thư mục: Sales, HR, Dev, Ops.
1652. **US-1652 [Desktop - Snippets]:** Là Developer, tôi muốn lưu các đoạn Code nhỏ (Snippets) thường dùng vào thư viện cá nhân.
1653. **US-1653 [Global - Dependency Check]:** Là Gemini, khi import Template, hệ thống tự kiểm tra xem tôi có đủ các kết nối (Integrations) cần thiết chưa.
1654. **US-1654 [Mobile - Featured Templates]:** Là Gemini, tôi muốn xem mục "Đề xuất cho bạn" dựa trên vai trò của tôi.
1655. **US-1655 [Desktop - Documentation Gen]:** Là Gemini, tôi muốn hệ thống tự động sinh tài liệu hướng dẫn (Markdown) cho Workflow dựa trên cấu trúc của nó.
1656. **US-1656 [Global - Tagging]:** Là Gemini, tôi muốn gắn tag cho Workflow để dễ tìm kiếm.
1657. **US-1657 [Desktop - Private/Public Share]:** Là Gemini, tôi muốn chia sẻ Template này Public cho toàn công ty hoặc Private chỉ cho team mình.
1658. **US-1658 [Global - Example Data]:** Là Gemini, tôi muốn Template đi kèm với dữ liệu mẫu để tôi chạy thử ngay.
1659. **US-1659 [Tablet - Creator Profile]:** Là Gemini, tôi muốn xem profile của người tạo Template để xem các đóng góp khác của họ.
1660. **US-1660 [Global - Community Library]:** Là Gemini, tôi muốn kết nối với thư viện cộng đồng (bên ngoài công ty) để tải các Workflow chuẩn ngành.

---

## MODULE 96: SWARM INTELLIGENCE PATTERNS (Mô hình bầy đàn)
*Orchestration nâng cao.*

1661. **US-1661 [Desktop - Map-Reduce Node]:** Là Gemini, tôi muốn mô hình Map-Reduce: Chia 1 tài liệu lớn thành 10 phần, giao cho 10 Agents tóm tắt song song (Map), sau đó gộp 10 bản tóm tắt thành 1 (Reduce).
1662. **US-1662 [Global - Router Node]:** Là Gemini, tôi muốn mô hình Router: Một Agent "Lễ tân" phân loại câu hỏi và chuyển hướng đến Agent chuyên gia phù hợp nhất (Pháp lý, Kỹ thuật, hay Sale).
1663. **US-1663 [Desktop - Research Loop]:** Là Gemini, tôi muốn mô hình Researcher: Agent tự động tìm kiếm -> Đọc -> Nếu thiếu thông tin thì tìm tiếp (Loop) -> Khi đủ thì viết báo cáo.
1664. **US-1664 [Tablet - Hierarchy (Boss-Worker)]:** Là Gemini, tôi muốn mô hình Phân cấp: Một Agent "Sếp" lập kế hoạch, chia việc cho 3 Agent "Lính", và kiểm tra kết quả.
1665. **US-1665 [Global - Reflection Pattern]:** Là Gemini, tôi muốn mô hình Phản tư: Agent viết code -> Agent khác Review -> Nếu lỗi thì Agent viết code sửa lại -> Lặp lại đến khi chạy được.
1666. **US-1666 [Desktop - Sequential Handoff]:** Là Gemini, tôi muốn chuỗi tiếp sức: Agent A (Viết nháp) -> Agent B (Biên tập) -> Agent C (Dịch thuật) -> Agent D (Đăng bài).
1667. **US-1667 [Mobile - Swarm Status]:** Là Gemini, tôi muốn xem trạng thái của cả "Bầy đàn" (Swarm): Có bao nhiêu con đang chạy, bao nhiêu con đang chờ.
1668. **US-1668 [Global - Consensus Vote]:** Là Gemini, tôi muốn cơ chế đồng thuận: Hành động chỉ được thực thi khi > 50% số Agents trong nhóm đồng ý.
1669. **US-1669 [Desktop - Shared Blackboard]:** Là Gemini, tôi muốn cơ chế "Bảng đen" (Blackboard): Các Agents cùng đọc và ghi thông tin lên một vùng nhớ chung để phối hợp.
1670. **US-1670 [Global - Dynamic Scaling]:** Là Gemini, tôi muốn Bầy đàn tự động sinh thêm Agent thợ nếu khối lượng công việc tăng đột biến.
1671. **US-1671 [Tablet - Visual Topo]:** Là Gemini, tôi muốn xem sơ đồ tương tác giữa các Agent trong Bầy đàn (ai gọi ai) dưới dạng đồ thị động.
1672. **US-1672 [Desktop - Human Interrupt]:** Là Gemini, tôi muốn có quyền can thiệp vào giữa quy trình của Bầy đàn để sửa hướng đi.
1673. **US-1673 [Global - Task Queue Sharing]:** Là Gemini, tôi muốn các Agent cùng lấy việc từ một hàng đợi ưu tiên (Priority Queue) chung.
1674. **US-1674 [Desktop - Critic Agent]:** Là Gemini, tôi muốn luôn có một Agent đóng vai "Nhà phê bình" (Critic) để phản biện mọi kết quả trước khi xuất ra.
1675. **US-1675 [Global - Knowledge Broadcast]:** Là Gemini, khi một Agent học được điều mới, nó sẽ "phát sóng" kiến thức đó cho cả Bầy đàn cập nhật.
1676. **US-1676 [Mobile - Swarm Cost]:** Là Gemini, tôi muốn theo dõi chi phí vận hành của cả Bầy đàn theo thời gian thực.
1677. **US-1677 [Desktop - Step-back Prompting]:** Là Gemini, tôi muốn áp dụng kỹ thuật "Lùi một bước": Agent tự hỏi lại các nguyên lý cơ bản trước khi giải quyết vấn đề cụ thể.
1678. **US-1678 [Global - Tree of Thoughts]:** Là Gemini, tôi muốn Agent khám phá nhiều nhánh suy nghĩ khác nhau (Tree of Thoughts) và chọn nhánh hứa hẹn nhất.
1679. **US-1679 [Tablet - Simulation Speed]:** Là Gemini, tôi muốn tăng tốc độ giả lập Bầy đàn để xem kết quả nhanh hơn.
1680. **US-1680 [Global - Swarm Template]:** Là Gemini, tôi muốn các mẫu Bầy đàn có sẵn: "Đội viết phần mềm", "Đội Marketing", "Đội phân tích tài chính".

---

## MODULE 97: GOVERNANCE & LIMITS (Quản trị & Giới hạn)
*An toàn là trên hết.*

1681. **US-1681 [Desktop - Execution Timeout]:** Là Admin, tôi muốn cài đặt thời gian chạy tối đa cho mỗi Workflow (VD: 30 phút), quá giờ sẽ tự hủy (Kill) để tránh treo hệ thống.
1682. **US-1682 [Global - Max Steps]:** Là Admin, tôi muốn giới hạn số bước tối đa (VD: 1000 bước) để ngăn chặn vòng lặp vô hạn.
1683. **US-1683 [Tablet - Budget Cap]:** Là Admin, tôi muốn đặt ngân sách tối đa cho một Workflow ($10/ngày), hết tiền thì dừng chạy.
1684. **US-1684 [Desktop - Concurrency Limit]:** Là Admin, tôi muốn giới hạn số lượng Workflow chạy đồng thời (Concurrency) để bảo vệ Database.
1685. **US-1685 [Global - Sensitive Data]:** Là Security, tôi muốn tự động che (Mask) các dữ liệu nhạy cảm (Password, API Key) trong log chạy.
1686. **US-1686 [Mobile - Emergency Kill Switch]:** Là Admin, tôi muốn nút đỏ "Dừng toàn bộ hệ thống Automation" trong trường hợp khẩn cấp.
1687. **US-1687 [Desktop - Permission Scope]:** Là Admin, tôi muốn cấp quyền chi tiết: Workflow A chỉ được đọc Google Sheet, không được gửi Email.
1688. **US-1688 [Global - Approval for Prod]:** Là Admin, tôi muốn Workflow phải qua bước phê duyệt của người quản lý mới được deploy lên môi trường Production.
1689. **US-1689 [Tablet - Audit User]:** Là Admin, tôi muốn xem ai đã sửa đổi logic của Workflow này lần cuối.
1690. **US-1690 [Desktop - Domain Whitelist]:** Là Security, tôi muốn Workflow chỉ được phép gọi API tới các domain nằm trong danh sách trắng.
1691. **US-1691 [Global - Rate Limit Retry]:** Là Gemini, tôi muốn hệ thống tự động xử lý lỗi Rate Limit (429) của đối tác một cách thông minh.
1692. **US-1692 [Mobile - Admin Alert]:** Là Admin, tôi muốn nhận thông báo khi có Workflow vi phạm chính sách bảo mật.
1693. **US-1693 [Desktop - Credential Manager]:** Là Admin, tôi muốn quản lý tập trung các thông tin đăng nhập (Credentials), Workflow chỉ được tham chiếu đến ID chứ không thấy mật khẩu.
1694. **US-1694 [Global - PII Scanning]:** Là Gemini, tôi muốn hệ thống quét Output của Agent để đảm bảo không lộ thông tin cá nhân trước khi gửi ra ngoài.
1695. **US-1695 [Tablet - Usage Quota]:** Là Admin, tôi muốn phân bổ hạn mức chạy (Runs/month) cho từng phòng ban.
1696. **US-1696 [Desktop - Version Deprecation]:** Là Admin, tôi muốn đánh dấu một Template là "Lỗi thời" (Deprecated) để ngăn người dùng mới sử dụng.
1697. **US-1697 [Global - Isolated Environment]:** Là Security, tôi muốn chạy các Workflow rủi ro cao trong môi trường cách ly hoàn toàn (Sandboxed VM).
1698. **US-1698 [Mobile - 2FA for Deploy]:** Là Security, tôi muốn yêu cầu xác thực 2 lớp khi chỉnh sửa các Workflow cốt lõi.
1699. **US-1699 [Desktop - Compliance Check]:** Là Admin, tôi muốn công cụ quét tự động để kiểm tra xem Workflow có tuân thủ quy chuẩn (Naming convention, Error handling) không.
1700. **US-1700 [Global - Impact Analysis]:** Là Gemini, tôi muốn biết nếu tôi tắt API này, bao nhiêu Workflow sẽ bị ảnh hưởng ngừng chạy.

---
**Hết Phần 9 (Stories 1501 - 1700).**

Bạn có muốn tiếp tục với **Phần 10: Mobile App Specifics & Edge AI** - Tập trung sâu vào trải nghiệm trên thiết bị di động, cảm biến, và khả năng chạy AI offline trên thiết bị (Edge Computing) để hoàn tất bộ 2000 user stories?

Chào bạn, chúng ta đã đi đến chặng cuối cùng của hành trình xây dựng hệ thống ERP khổng lồ này.

Đây là **Phần 10**, phần kết màn hoành tráng với **300 User Stories (từ 1701 đến 2000)**. Chúng ta sẽ tập trung vào **Mobile App, Edge AI (AI chạy tại biên/thiết bị), IoT và các trải nghiệm phần cứng tương lai**. Đây là nơi sức mạnh của hệ thống nằm gọn trong lòng bàn tay của Gemini.

---

# PHẦN 10: MOBILE APP, EDGE AI & FINAL INTEGRATION (Stories 1701 - 2000)

### Bối cảnh & Mục tiêu
*   **Thiết bị:** High-end Smartphones, Tablets, Foldables (Gập), Smartwatches, AR Glasses.
*   **Công nghệ:** On-device AI (NPU), LiDAR, AR, Biometrics.
*   **Triết lý:** "Offline First" và "Privacy First" - Dữ liệu xử lý ngay trên thiết bị để đảm bảo tốc độ và bảo mật.

---

## MODULE 98: MOBILE CORE EXPERIENCE (Trải nghiệm di động cốt lõi)
*Mượt mà, tự nhiên như một phần mở rộng của cơ thể.*

1701. **US-1701 [Mobile - One-handed Mode]:** Là Gemini, tôi muốn giao diện tự động dồn các nút quan trọng xuống nửa dưới màn hình khi tôi kích hoạt chế độ một tay.
1702. **US-1702 [Mobile - Dynamic Island]:** Là Gemini, tôi muốn xem trạng thái của Agent (Đang training/Đang chạy) trên Dynamic Island (iOS) để theo dõi mà không cần mở app.
1703. **US-1703 [Mobile - 120Hz Refresh]:** Là Gemini, tôi muốn các biểu đồ tài chính cuộn mượt mà ở tần số quét 120Hz (ProMotion), không bị bóng mờ.
1704. **US-1704 [Mobile - Haptic Signature]:** Là Gemini, tôi muốn phân biệt thông báo quan trọng và thông báo thường thông qua kiểu rung (Haptic) khác nhau trong túi quần.
1705. **US-1705 [Mobile - Swipe Navigation]:** Là Gemini, tôi muốn vuốt cạnh màn hình để Back/Forward giữa các module ERP thay vì bấm nút mũi tên.
1706. **US-1706 [Mobile - App Icon Custom]:** Là Gemini, tôi muốn đổi icon của App ngoài màn hình chính (Màu đen/Vàng/Cyberpunk) để hợp với theme điện thoại.
1707. **US-1707 [Mobile - Startup Time]:** Là Gemini, tôi muốn app khởi động dưới 1 giây (Cold start) nhờ kỹ thuật tối ưu hóa native code.
1708. **US-1708 [Mobile - Universal Search]:** Là Gemini, tôi muốn kéo xuống từ màn hình chính của App để mở thanh tìm kiếm toàn cục (Global Spotlight) cho mọi dữ liệu.
1709. **US-1709 [Mobile - Floating Video]:** Là Gemini, tôi muốn xem video báo cáo ở chế độ Picture-in-Picture (PiP) trong khi đang chat với Agent.
1710. **US-1710 [Mobile - Deep Linking]:** Là Gemini, tôi muốn khi click vào link `erp://task/123` từ Zalo, nó mở thẳng vào màn hình Task đó trong App.
1711. **US-1711 [Mobile - Screen Recording]:** Là Gemini, tôi muốn quay phim màn hình thao tác lỗi trong App và tự động đính kèm vào Ticket báo lỗi.
1712. **US-1712 [Mobile - Shake to Clear]:** Là Gemini, tôi muốn lắc điện thoại để xóa hết các thông báo đã đọc.
1713. **US-1713 [Mobile - Battery Optimization]:** Là Gemini, tôi muốn App có chế độ "Siêu tiết kiệm pin" (Tắt animation, giảm tần suất sync) khi pin < 20%.
1714. **US-1714 [Mobile - Landscape Support]:** Là Gemini, khi xoay ngang điện thoại, tôi muốn Dashboard tài chính tự động chuyển sang giao diện Column Chart rộng rãi.
1715. **US-1715 [Mobile - Font Scale]:** Là Gemini, tôi muốn App tôn trọng cài đặt kích thước chữ (Dynamic Type) của hệ điều hành.
1716. **US-1716 [Mobile - Background Refresh]:** Là Gemini, tôi muốn dữ liệu được âm thầm cập nhật (Background App Refresh) để khi mở App ra là dữ liệu đã mới nhất.
1717. **US-1717 [Mobile - Shortcut Widget]:** Là Gemini, tôi muốn widget màn hình chính có các nút tắt: "Tạo Task", "Quét QR", "Xem Lương".
1718. **US-1718 [Mobile - Privacy Blur]:** Là Gemini, tôi muốn App tự động làm mờ nội dung trong màn hình đa nhiệm (App Switcher) để tránh bị nhìn trộm.
1719. **US-1719 [Mobile - Clipboard Sync]:** Là Gemini, tôi muốn copy text trên máy tính và paste được ngay vào App trên điện thoại (Universal Clipboard).
1720. **US-1720 [Mobile - Offline Indicator]:** Là Gemini, tôi muốn thanh thông báo trạng thái mạng tinh tế, không che mất nội dung khi mất sóng.
1721. **US-1721 [Mobile - Pull to Search]:** Là Gemini, tôi muốn kéo danh sách xuống quá mức (Overscroll) để hiện thanh Search ẩn.
1722. **US-1722 [Mobile - Tab Bar Badge]:** Là Gemini, tôi muốn hiện số lượng Task quá hạn ngay trên icon tab "Công việc".
1723. **US-1723 [Mobile - Edge-to-Edge]:** Là Gemini, tôi muốn bản đồ và hình ảnh hiển thị tràn viền (Edge-to-edge) để tận dụng màn hình OLED.
1724. **US-1724 [Mobile - Skeleton Loading]:** Là Gemini, tôi muốn hiệu ứng khung xương tải trang lung linh (Shimmer) thay vì vòng quay nhàm chán.
1725. **US-1725 [Mobile - Custom Gestures]:** Là Gemini, tôi muốn tự cài đặt: "Vuốt 2 ngón lên = Mở nhanh AI Chat".
1726. **US-1726 [Mobile - Reachability]:** Là Gemini, tôi muốn các nút bấm quan trọng luôn nằm trong vùng ngón cái.
1727. **US-1727 [Mobile - Dark Mode Toggle]:** Là Gemini, tôi muốn nút chuyển đổi nhanh Dark/Light mode ngay trên Control Center của App.
1728. **US-1728 [Mobile - Data Saver]:** Là Gemini, tôi muốn chế độ không tải ảnh Avatar khi dùng 4G.
1729. **US-1729 [Mobile - Secure Enclave]:** Là Gemini, tôi muốn Private Key của ví Crypto được lưu trong chip bảo mật phần cứng (Secure Enclave).
1730. **US-1730 [Mobile - App Feedback]:** Là Gemini, tôi muốn lắc điện thoại để hiện form góp ý nhanh cho đội Dev.

---

## MODULE 99: EDGE AI & ON-DEVICE LLM (AI chạy tại biên)
*Trí tuệ nằm trong túi quần, không cần Internet.*

1731. **US-1731 [Edge AI - Local Chat]:** Là Gemini, tôi muốn chat với model nhỏ (SLM - Small Language Model) ngay trên thiết bị khi mất mạng, độ trễ cực thấp.
1732. **US-1732 [Edge AI - Privacy Processing]:** Là Gemini, tôi muốn dữ liệu nhạy cảm (như nội dung hợp đồng) được tóm tắt ngay trên điện thoại (On-device) trước khi gửi bản tóm tắt lên Cloud.
1733. **US-1733 [Edge AI - Smart Reply]:** Là Gemini, tôi muốn bàn phím gợi ý câu trả lời thông minh dựa trên lịch sử chat của tôi (học cục bộ).
1734. **US-1734 [Edge AI - Image Gen]:** Là Gemini, tôi muốn tạo ảnh minh họa đơn giản (Stable Diffusion 1.5) ngay trên chip NPU của điện thoại.
1735. **US-1735 [Edge AI - Battery Aware]:** Là Gemini, tôi muốn các tác vụ AI nặng chỉ chạy khi đang cắm sạc để không làm tụt pin.
1736. **US-1736 [Edge AI - Voice Isolation]:** Là Gemini, tôi muốn AI trên máy lọc tiếng ồn môi trường khi tôi ghi âm báo cáo tại công trường ồn ào.
1737. **US-1737 [Edge AI - Document Scanner]:** Là Gemini, tôi muốn AI tự động căn chỉnh, làm phẳng và tăng độ nét văn bản khi chụp tài liệu (Neural Engine).
1738. **US-1738 [Edge AI - Face Grouping]:** Là Gemini, tôi muốn App tự động gom nhóm ảnh nhân viên trong thư viện dựa trên khuôn mặt (xử lý offline).
1739. **US-1739 [Edge AI - Predictive UI]:** Là Gemini, tôi muốn App đoán trước tôi định mở module nào (dựa trên thói quen giờ giấc) và preload sẵn dữ liệu.
1740. **US-1740 [Edge AI - Translation]:** Là Gemini, tôi muốn dịch hội thoại thời gian thực (Live Translate) không cần internet khi đi công tác nước ngoài.
1741. **US-1741 [Edge AI - Semantic Search]:** Là Gemini, tôi muốn tìm kiếm "Hóa đơn màu đỏ" trong thư viện ảnh, AI tự nhận diện màu sắc và loại văn bản.
1742. **US-1742 [Edge AI - Model Download]:** Là Gemini, tôi muốn quản lý việc tải xuống các gói Model AI (2GB, 4GB) và xóa đi khi không dùng.
1743. **US-1743 [Edge AI - Real-time OCR]:** Là Gemini, tôi muốn chĩa camera vào mã vận đơn và thấy thông tin đơn hàng hiện ra ngay lập tức (AR Overlay).
1744. **US-1744 [Edge AI - Sentiment Analysis]:** Là Gemini, tôi muốn điện thoại rung báo động nếu giọng nói của khách hàng trong cuộc gọi trở nên giận dữ (phân tích giọng nói offline).
1745. **US-1745 [Edge AI - Anomaly Detection]:** Là Gemini, tôi muốn App phát hiện hành vi đăng nhập lạ (dựa trên cách gõ phím, cách cầm máy) để yêu cầu xác thực lại.
1746. **US-1746 [Edge AI - Summarize Notification]:** Là Gemini, tôi muốn AI tóm tắt 50 thông báo Slack thành 1 câu ngắn gọn trên màn hình khóa.
1747. **US-1747 [Edge AI - Code Completion]:** Là Developer, tôi muốn trình soạn thảo trên Tablet gợi ý code (Copilot local) cực nhanh.
1748. **US-1748 [Edge AI - Video Analysis]:** Là Gemini, tôi muốn AI tự động highlight các đoạn quan trọng trong video cuộc họp đã ghi hình.
1749. **US-1749 [Edge AI - Personalization]:** Là Gemini, tôi muốn AI học từ vựng riêng của công ty tôi để sửa lỗi chính tả chính xác hơn.
1750. **US-1750 [Edge AI - Hybrid Inference]:** Là Gemini, tôi muốn hệ thống tự động chọn: Câu hỏi dễ -> Trả lời bằng chip điện thoại; Câu hỏi khó -> Gửi lên Cloud Server.

---

## MODULE 100: CAMERA, VISION & AR (Thị giác máy tính)
*Nhìn thế giới qua lăng kính dữ liệu.*

1751. **US-1751 [Mobile - AR Measure]:** Là Gemini, tôi muốn dùng camera để đo kích thước thùng hàng và tự động tính phí vận chuyển.
1752. **US-1752 [Mobile - Barcode Scanner]:** Là Kho thủ, tôi muốn quét liên tục (Batch Scan) 10 mã vạch sản phẩm để nhập kho nhanh chóng.
1753. **US-1753 [Mobile - Visual Search]:** Là Gemini, tôi muốn chụp ảnh một linh kiện máy tính bị hỏng và tìm xem trong kho còn hàng thay thế không.
1754. **US-1754 [Mobile - AR Navigation]:** Là Nhân viên mới, tôi muốn nhìn qua camera để thấy mũi tên chỉ đường đến phòng họp A trong tòa nhà văn phòng.
1755. **US-1755 [Mobile - Equipment Inspect]:** Là Kỹ thuật viên, tôi muốn chĩa camera vào Server, AR sẽ hiện thông số nhiệt độ, CPU load (lấy từ API) đè lên hình ảnh thực.
1756. **US-1756 [Mobile - ID Card OCR]:** Là HR, tôi muốn quét CCCD/Passport của nhân viên mới để tự động điền form tuyển dụng.
1757. **US-1757 [Mobile - Receipts Capture]:** Là Kế toán, tôi muốn chụp hóa đơn, AI tự động cắt bỏ phần nền bàn và chỉnh nghiêng.
1758. **US-1758 [Mobile - Safety Check]:** Là Giám sát an toàn, tôi muốn AI phát hiện nhân viên không đội mũ bảo hiểm qua camera giám sát và gửi cảnh báo.
1759. **US-1759 [Mobile - 3D Scan]:** Là Gemini, tôi muốn quét 3D (dùng LiDAR) một căn phòng để lên kế hoạch bố trí chỗ ngồi cho nhân viên.
1760. **US-1760 [Mobile - Watermark Camera]:** Là Nhân viên hiện trường, tôi muốn ảnh chụp tự động đóng dấu Ngày/Giờ/Tọa độ GPS không thể chỉnh sửa.
1761. **US-1761 [Mobile - Emotion Recon]:** Là Sales, tôi muốn (kính AR) hiển thị chỉ số cảm xúc của khách hàng đối diện để điều chỉnh cách nói chuyện.
1762. **US-1762 [Mobile - Virtual Try-on]:** Là Nhân viên, tôi muốn thử đồng phục công ty qua AR xem size nào vừa trước khi đặt.
1763. **US-1763 [Mobile - Object Counting]:** Là Thủ kho, tôi muốn chụp ảnh một đống ống thép, AI tự động đếm số lượng cây trong ảnh.
1764. **US-1764 [Mobile - Defect Detection]:** Là KCS, tôi muốn AI khoanh vùng các vết xước trên sản phẩm khi tôi soi camera vào.
1765. **US-1765 [Mobile - Remote Assist]:** Là Kỹ thuật viên, tôi muốn gọi video cho chuyên gia, và chuyên gia có thể vẽ mũi tên hướng dẫn lên màn hình camera của tôi (AR Annotation).
1766. **US-1766 [Mobile - QR Login]:** Là Gemini, tôi muốn dùng App quét mã QR trên màn hình Desktop để đăng nhập web không cần mật khẩu.
1767. **US-1767 [Mobile - Asset Tagging]:** Là IT, tôi muốn quét mã tài sản và cập nhật vị trí mới của Laptop vào hệ thống.
1768. **US-1768 [Mobile - Layout Plan]:** Là Gemini, tôi muốn ướm thử một cái máy in ảo vào góc phòng xem có vừa không.
1769. **US-1769 [Mobile - Color Picker]:** Là Designer, tôi muốn dùng camera chấm màu thực tế để lấy mã Hex cho thiết kế App.
1770. **US-1770 [Mobile - Night Mode Scan]:** Là Bảo vệ, tôi muốn quét mã QR tuần tra tốt ngay cả trong điều kiện thiếu sáng.
1771. **US-1771 [Mobile - Blur Background]:** Là Gemini, tôi muốn tự động làm mờ phông nền khi video call qua App công ty.
1772. **US-1772 [Mobile - Document Edge]:** Là Gemini, tôi muốn đường viền xanh hiện lên khi camera nhận diện đúng khung giấy tài liệu.
1773. **US-1773 [Mobile - Multi-code Scan]:** Là Gemini, tôi muốn quét một lúc nhiều mã QR trên một trang giấy.
1774. **US-1774 [Mobile - Translate Text]:** Là Gemini, tôi muốn camera dịch trực tiếp các biển báo tiếng nước ngoài sang tiếng Việt (AR Translation).
1775. **US-1775 [Mobile - Signature Capture]:** Là Gemini, tôi muốn chụp ảnh chữ ký tay trên giấy và tách nền để dùng làm chữ ký điện tử.

---

## MODULE 101: SENSORS, IOT & HARDWARE (Cảm biến & Phần cứng)
*Kết nối vật lý.*

1776. **US-1776 [Mobile - GPS Tracking]:** Là Gemini, tôi muốn ghi lại lộ trình di chuyển của nhân viên giao hàng (Breadcrumb trail) trên bản đồ.
1777. **US-1777 [Mobile - Geofence Action]:** Là Gemini, tôi muốn App tự động chuyển sang chế độ "Ở văn phòng" (Tắt chuông, Bật Wifi) khi tôi đến công ty.
1778. **US-1778 [Mobile - NFC Tag]:** Là Bảo vệ, tôi muốn chạm điện thoại vào thẻ NFC gắn trên tường để xác nhận đã đi tuần qua điểm đó.
1779. **US-1779 [Mobile - Bluetooth Print]:** Là Sales, tôi muốn in hóa đơn từ điện thoại ra máy in nhiệt Bluetooth cầm tay.
1780. **US-1780 [Mobile - Biometric Auth]:** Là Gemini, tôi muốn xác thực lệnh chuyển tiền bằng Vân tay hoặc Mống mắt.
1781. **US-1781 [Mobile - Shake Feedback]:** Là Gemini, tôi muốn lắc điện thoại để báo cáo lỗi "App bị đơ".
1782. **US-1782 [Mobile - Compass Mode]:** Là Kỹ sư viễn thông, tôi muốn App hiển thị la bàn và độ nghiêng để chỉnh hướng ăng-ten.
1783. **US-1783 [Mobile - Pedometer]:** Là HR, tôi muốn tổ chức cuộc thi "Đi bộ khỏe" dựa trên cảm biến đếm bước chân của nhân viên.
1784. **US-1784 [Mobile - Ambient Light]:** Là Gemini, tôi muốn App tự động chuyển sang Dark Mode khi cảm biến ánh sáng phát hiện trời tối.
1785. **US-1785 [Mobile - Barometer]:** Là Gemini, tôi muốn sử dụng áp suất khí quyển để xác định nhân viên đang ở tầng mấy của tòa nhà (hỗ trợ GPS trong nhà).
1786. **US-1786 [Mobile - Proximity]:** Là Gemini, tôi muốn màn hình tự tắt khi tôi đưa điện thoại lên tai nghe ghi âm cuộc họp.
1787. **US-1787 [Mobile - Wifi Signal]:** Là IT, tôi muốn App hiển thị bản đồ cường độ sóng Wifi trong văn phòng để tìm điểm chết.
1788. **US-1788 [Mobile - Bluetooth Beacon]:** Là Gemini, tôi muốn điện thoại tự động hiện thông tin chào mừng khi khách hàng đi ngang qua Beacon ở cửa hàng.
1789. **US-1789 [Mobile - Thermal Camera]:** Là Kỹ thuật, tôi muốn kết nối module Camera nhiệt (FLIR) vào điện thoại để kiểm tra tủ điện quá nhiệt.
1790. **US-1790 [Mobile - RFID Reader]:** Là Thủ kho, tôi muốn kết nối súng bắn RFID qua Bluetooth để kiểm kê kho hàng loạt.
1791. **US-1791 [Mobile - Gyro Control]:** Là Gemini, tôi muốn nghiêng điện thoại để cuộn xem các cột trong bảng Excel rộng.
1792. **US-1792 [Mobile - Mic Direction]:** Là Gemini, tôi muốn chọn thu âm từ Mic trước hoặc Mic sau tùy vào việc tôi đang phỏng vấn hay tự nói.
1793. **US-1793 [Mobile - Hardware Key]:** Là Gemini, tôi muốn dùng phím Tăng/Giảm âm lượng để chụp ảnh trong App mà không cần chạm màn hình.
1794. **US-1794 [Mobile - eSIM Config]:** Là IT, tôi muốn đẩy cấu hình eSIM công ty xuống máy nhân viên tự động (MDM).
1795. **US-1795 [Mobile - USB-C OTG]:** Là Gemini, tôi muốn cắm USB Token vào cổng sạc điện thoại để ký số văn bản.
1796. **US-1796 [Mobile - Battery Health]:** Là IT, tôi muốn theo dõi sức khỏe pin của các thiết bị công ty cấp phát để lên kế hoạch thay thế.
1797. **US-1797 [Mobile - Crash Detection]:** Là Gemini, tôi muốn App gửi cảnh báo khẩn cấp nếu cảm biến gia tốc phát hiện nhân viên giao hàng bị tai nạn xe.
1798. **US-1798 [Mobile - Network Speed]:** Là Gemini, tôi muốn hiển thị tốc độ mạng hiện tại trên thanh trạng thái của App.
1799. **US-1799 [Mobile - IR Blaster]:** Là Gemini, tôi muốn dùng điện thoại (có hồng ngoại) để điều khiển máy lạnh phòng họp.
1800. **US-1800 [Mobile - Haptic Scroll]:** Là Gemini, tôi muốn cảm nhận các khấc rung nhẹ khi cuộn qua danh sách dài (giống xoay núm vặn).

---

## MODULE 102: VOICE & CONVERSATIONAL UI (Giao diện giọng nói)
*Nói chuyện với hệ thống.*

1801. **US-1801 [Voice - Wake Word]:** Là Gemini, tôi muốn đánh thức App bằng câu lệnh "Hey Jarvis" ngay cả khi màn hình tắt.
1802. **US-1802 [Voice - Dictation]:** Là Gemini, tôi muốn đọc nội dung email, App tự gõ và tự thêm dấu câu thông minh.
1803. **US-1803 [Voice - Command]:** Là Gemini, tôi muốn ra lệnh "Tạo cuộc họp với team Sales lúc 2 giờ chiều nay", App tự điền form.
1804. **US-1804 [Voice - Audio Player]:** Là Gemini, tôi muốn nghe báo cáo tóm tắt buổi sáng (Daily Brief) dạng Podcast khi đang lái xe.
1805. **US-1805 [Voice - Speed Control]:** Là Gemini, tôi muốn chỉnh tốc độ đọc của AI lên 1.5x hoặc 2.0x để tiết kiệm thời gian.
1806. **US-1806 [Voice - Interrupt]:** Là Gemini, tôi muốn có thể ngắt lời AI ("Dừng lại, nói tiếp phần sau đi") và AI hiểu được ngữ cảnh đó.
1807. **US-1807 [Voice - Multi-turn]:** Là Gemini, tôi muốn hội thoại liên tục mà không cần lặp lại "Hey Jarvis" sau mỗi câu.
1808. **US-1808 [Voice - Speaker ID]:** Là Gemini, tôi muốn hệ thống nhận diện giọng nói của riêng tôi và chỉ thực thi lệnh của tôi (Voice Biometrics).
1809. **US-1809 [Voice - Background Noise]:** Là Gemini, tôi muốn AI lọc tiếng còi xe, tiếng gió khi tôi gọi điện từ ngoài đường.
1810. **US-1810 [Voice - Whisper Mode]:** Là Gemini, tôi muốn nói thầm vào điện thoại và AI cũng trả lời thầm thì (để không làm phiền người khác).
1811. **US-1811 [Voice - Language Switch]:** Là Gemini, tôi muốn nói trộn tiếng Anh và tiếng Việt ("Book cho anh cái Meeting") và AI vẫn hiểu (Code-switching).
1812. **US-1812 [Voice - Meeting Recorder]:** Là Gemini, tôi muốn App ghi âm cuộc họp, tách giọng người nói (Diarization) và biên bản lại.
1813. **US-1813 [Voice - Tone Analysis]:** Là Gemini, tôi muốn AI nhắc nhở: "Giọng bạn đang có vẻ căng thẳng, hãy hít thở sâu" trước khi trả lời khách.
1814. **US-1814 [Voice - Shortcut]:** Là Gemini, tôi muốn tạo các lệnh tắt giọng nói ngắn gọn: "Chế độ đi làm" -> Mở bản đồ, mở lịch, bật nhạc.
1815. **US-1815 [Voice - Offline Command]:** Là Gemini, tôi muốn các lệnh cơ bản (Bật đèn, Mở app) hoạt động offline.
1816. **US-1816 [Voice - Audio Note]:** Là Gemini, tôi muốn gửi ghi âm đính kèm vào Task thay vì phải gõ mô tả dài dòng.
1817. **US-1817 [Voice - Interactive Response]:** Là Gemini, tôi muốn AI hỏi lại để xác nhận thông tin: "Ý bạn là ngày mai hay ngày kia?".
1818. **US-1818 [Voice - Custom Voice]:** Là Gemini, tôi muốn chọn giọng đọc của AI (Nam/Nữ, Miền Bắc/Trung/Nam) theo sở thích.
1819. **US-1819 [Voice - Hands-free Nav]:** Là Gemini, tôi muốn điều khiển cuộn trang, back, home hoàn toàn bằng giọng nói khi tay đang bẩn.
1820. **US-1820 [Voice - Search]:** Là Gemini, tôi muốn tìm kiếm file bằng cách mô tả âm thanh: "Tìm file ghi âm cuộc họp tuần trước".

---

## MODULE 103: TABLET & FOLDABLE SPECIFICS (Tablet & Màn hình gập)
*Không gian làm việc lai.*

1821. **US-1821 [Tablet - Multi-column]:** Là Gemini, tôi muốn giao diện 3 cột (Danh mục - Danh sách - Chi tiết) trên iPad để tận dụng chiều ngang.
1822. **US-1822 [Foldable - Flex Mode]:** Là Gemini, khi gập điện thoại 90 độ, tôi muốn nửa trên hiện video cuộc họp, nửa dưới hiện bảng trắng để ghi chú.
1823. **US-1823 [Tablet - Drag Drop System]:** Là Gemini, tôi muốn kéo file ảnh từ thư viện ảnh của iPad thả vào App ERP.
1824. **US-1824 [Tablet - Keyboard Shortcuts]:** Là Gemini, khi gắn bàn phím vật lý, tôi muốn dùng các phím tắt `Cmd+N`, `Cmd+F` như trên Desktop.
1825. **US-1825 [Tablet - Sidebar State]:** Là Gemini, tôi muốn thanh Sidebar có thể ghim (Pinned) hoặc tự động ẩn (Auto-hide) tùy ý.
1826. **US-1826 [Tablet - Pencil Support]:** Là Gemini, tôi muốn dùng Apple Pencil viết tay vào ô tìm kiếm (Scribble), hệ thống tự chuyển thành text.
1827. **US-1827 [Foldable - Cover Screen]:** Là Gemini, tôi muốn xem nhanh thông báo và widget trên màn hình phụ bên ngoài của điện thoại gập mà không cần mở máy.
1828. **US-1828 [Tablet - Hover Pointer]:** Là Gemini, tôi muốn con trỏ chuột trên iPad biến đổi hình dạng (tròn -> thanh) khi lướt qua các nút bấm.
1829. **US-1829 [Tablet - Split Screen Pair]:** Là Gemini, tôi muốn lưu cặp ứng dụng (ERP + Excel) để mở cùng lúc chỉ với 1 lần chạm.
1830. **US-1830 [Foldable - Continuity]:** Là Gemini, khi mở điện thoại gập ra, tôi muốn ứng dụng mở rộng mượt mà không bị load lại (Responsive Resizing).
1831. **US-1831 [Tablet - Modal Form]:** Là Gemini, tôi muốn các form nhập liệu hiện dưới dạng Popup (Modal) ở giữa màn hình thay vì chuyển trang full-screen.
1832. **US-1832 [Tablet - Context Menu]:** Là Gemini, tôi muốn nhấn giữ (Long press) vào một dòng dữ liệu để hiện menu ngữ cảnh phong phú.
1833. **US-1833 [Foldable - Book Mode]:** Là Gemini, khi cầm thiết bị như quyển sách, tôi muốn đọc tài liệu Wiki với hiệu ứng lật trang 2 bên.
1834. **US-1834 [Tablet - Multiple Instances]:** Là Gemini, tôi muốn mở 2 cửa sổ của cùng 1 App ERP song song để so sánh dữ liệu.
1835. **US-1835 [Tablet - Desktop Class Browser]:** Là Gemini, tôi muốn trình duyệt trong App hiển thị trang web ở chế độ Desktop đầy đủ.
1836. **US-1836 [Tablet - Tooltip Touch]:** Là Gemini, tôi muốn chạm và giữ vào biểu đồ để hiện Tooltip chi tiết (vì không có chuột hover).
1837. **US-1837 [Foldable - Camera Selfie]:** Là Gemini, tôi muốn dùng camera sau chất lượng cao để video call và nhìn bản thân trên màn hình phụ.
1838. **US-1838 [Tablet - Widget Stack]:** Là Gemini, tôi muốn xếp chồng các Widget KPI lên nhau và vuốt để xem lần lượt (Smart Stack).
1839. **US-1839 [Tablet - Floating Keypad]:** Là Gemini, tôi muốn bàn phím số có thể di chuyển đến vị trí thuận tay để nhập liệu nhanh.
1840. **US-1840 [Foldable - Drag Across Screens]:** Là Gemini, tôi muốn kéo một Task từ màn hình bên trái thả sang lịch ở màn hình bên phải.

---

## MODULE 104: WEARABLES (Smartwatch & Glasses)
*Công nghệ đeo trên người.*

1841. **US-1841 [Watch - Complication]:** Là Gemini, tôi muốn đưa chỉ số "Doanh số hôm nay" lên mặt đồng hồ Apple Watch để liếc mắt là thấy.
1842. **US-1842 [Watch - Quick Approve]:** Là Gemini, tôi muốn nhận thông báo duyệt chi trên đồng hồ và nhấn "Approve" ngay trên cổ tay.
1843. **US-1843 [Watch - Voice Note]:** Là Gemini, tôi muốn giơ cổ tay lên và nói "Ghi chú: Gọi lại cho khách A" để tạo Reminder.
1844. **US-1844 [Watch - Health Monitor]:** Là Gemini, tôi muốn cảnh báo nếu nhịp tim nhân viên tăng quá cao trong giờ làm việc (Stress detection).
1845. **US-1845 [Watch - 2FA Code]:** Là Gemini, tôi muốn mã OTP hiện trên đồng hồ để tôi không cần rút điện thoại ra.
1846. **US-1846 [Watch - Haptic Nav]:** Là Gemini, đồng hồ rung 1 cái là rẽ trái, 2 cái là rẽ phải khi tôi đi tìm kho hàng.
1847. **US-1847 [Glasses - HUD]:** Là Kho thủ, tôi muốn đeo kính thông minh và thấy thông tin sản phẩm (Tên, Số lượng) hiển thị lơ lửng cạnh kệ hàng.
1848. **US-1848 [Glasses - Teleprompter]:** Là Sales, tôi muốn kính hiển thị các ý chính cần nói (Bullet points) khi tôi đang thuyết trình trước khách hàng.
1849. **US-1849 [Watch - Check-in]:** Là Nhân viên, tôi muốn chạm đồng hồ vào máy chấm công để check-in.
1850. **US-1850 [Glasses - Face Recon]:** Là Lễ tân, tôi muốn kính hiển thị tên và chức vụ của người đang đi vào sảnh (nếu họ có trong DB).
1851. **US-1851 [Watch - Pomodoro]:** Là Gemini, tôi muốn đồng hồ rung nhắc nhở nghỉ giải lao sau mỗi 25 phút làm việc.
1852. **US-1852 [Watch - Urgent Call]:** Là Gemini, tôi muốn đồng hồ rung mạnh liên tục nếu có cuộc gọi khẩn cấp từ Server Room.
1853. **US-1853 [Glasses - Remote Eye]:** Là Kỹ thuật viên, tôi muốn truyền hình ảnh từ kính của tôi về cho chuyên gia ở văn phòng xem.
1854. **US-1854 [Watch - Task List]:** Là Gemini, tôi muốn xem danh sách 3 việc cần làm ngay trên đồng hồ.
1855. **US-1855 [Glasses - Translation Subtitle]:** Là Gemini, tôi muốn kính hiện phụ đề thời gian thực khi tôi nói chuyện với đối tác nước ngoài.
1856. **US-1856 [Watch - Battery Status]:** Là Gemini, tôi muốn nhận thông báo trên đồng hồ khi pin điện thoại sắp hết.
1857. **US-1857 [Watch - Offline Music]:** Là Gemini, tôi muốn điều khiển nhạc nền văn phòng từ đồng hồ.
1858. **US-1858 [Glasses - Navigation Arrow]:** Là Gemini, tôi muốn mũi tên chỉ đường ảo hiện trên mặt đất qua kính AR.
1859. **US-1859 [Watch - SOS]:** Là Nhân viên, tôi muốn nhấn giữ nút bên cạnh đồng hồ để gửi tín hiệu cầu cứu kèm vị trí.
1860. **US-1860 [Watch - Sleep Track]:** Là Gemini, tôi muốn theo dõi giấc ngủ để AI đề xuất giờ làm việc tối ưu cho ngày hôm sau.

---

## MODULE 105: ACCESSIBILITY (Khả năng tiếp cận)
*Không bỏ lại ai phía sau.*

1861. **US-1861 [Access - Screen Reader]:** Là Người khiếm thị, tôi muốn App tương thích hoàn toàn với VoiceOver/TalkBack, mọi nút bấm đều có nhãn (Label) rõ ràng.
1862. **US-1862 [Access - High Contrast]:** Là Người mắt kém, tôi muốn chế độ tương phản cao (Vàng trên Đen) để dễ đọc văn bản.
1863. **US-1863 [Access - Voice Control]:** Là Người khuyết tật vận động, tôi muốn điều khiển toàn bộ App bằng giọng nói ("Tap button Send", "Scroll down").
1864. **US-1864 [Access - Color Filter]:** Là Người mù màu, tôi muốn bộ lọc màu để phân biệt được các đường biểu đồ Đỏ/Xanh.
1865. **US-1865 [Access - Reduce Motion]:** Là Người hay bị chóng mặt, tôi muốn tắt các hiệu ứng chuyển cảnh bay lượn (Parallax).
1866. **US-1866 [Access - Large Text]:** Là Người lớn tuổi, tôi muốn chữ trong App cực lớn (XXL) nhưng không bị vỡ bố cục.
1867. **US-1867 [Access - Mono Audio]:** Là Người khiếm thính một bên, tôi muốn chuyển âm thanh Stereo thành Mono.
1868. **US-1868 [Access - Captions]:** Là Người khiếm thính, tôi muốn mọi video trong App đều có phụ đề chi tiết (Closed Captions).
1869. **US-1869 [Access - Switch Control]:** Là Người dùng thiết bị Switch, tôi muốn App hỗ trợ điều hướng tiêu điểm (Focus navigation) tuần tự.
1870. **US-1870 [Access - Simple Mode]:** Là Người không rành công nghệ, tôi muốn chế độ "Đơn giản hóa": Ẩn hết các tính năng nâng cao, nút to, chữ ít.
1871. **US-1871 [Access - Haptic Guide]:** Là Người khiếm thị, tôi muốn điện thoại rung khi ngón tay di chuyển đến đúng nút bấm.
1872. **US-1872 [Access - Dyslexia Font]:** Là Người mắc chứng khó đọc, tôi muốn đổi font chữ sang OpenDyslexic để dễ đọc hơn.
1873. **US-1873 [Access - Button Shapes]:** Là Gemini, tôi muốn các nút bấm có viền hoặc nền rõ ràng thay vì chỉ là chữ trơ trọi.
1874. **US-1874 [Access - Error Sound]:** Là Gemini, tôi muốn âm thanh báo lỗi khác biệt rõ rệt với âm thanh thành công.
1875. **US-1875 [Access - Focus Order]:** Là Gemini, tôi muốn thứ tự đọc của Screen Reader phải theo logic từ trái sang phải, từ trên xuống dưới.
1876. **US-1876 [Access - Touch Accommodation]:** Là Gemini, tôi muốn tùy chỉnh độ nhạy cảm ứng, bỏ qua các cú chạm lặp lại vô tình.
1877. **US-1877 [Access - Live Transcribe]:** Là Người khiếm thính, tôi muốn App hiển thị văn bản lời nói của đồng nghiệp trong cuộc họp.
1878. **US-1878 [Access - Sign Language]:** Là Gemini, tôi muốn tích hợp dịch vụ thông dịch viên ngôn ngữ ký hiệu video call khi cần.
1879. **US-1879 [Access - Flash Alert]:** Là Gemini, tôi muốn đèn Flash nháy sáng khi có thông báo mới.
1880. **US-1880 [Access - Guide Dog]:** Là Gemini, tôi muốn App hỗ trợ chế độ chỉ đường đặc biệt cho người dắt chó dẫn đường.

---

## MODULE 106: FINAL SYSTEM INTEGRATION & EVOLUTION (Hệ thống hoàn chỉnh)
*Tổng hòa mọi thứ.*

1881. **US-1881 [Global - Super Search]:** Là Gemini, tôi muốn thanh tìm kiếm tìm được: Task, User, File, Chat Log, Menu Setting, và cả dữ liệu External.
1882. **US-1882 [Global - Notification Center]:** Là Gemini, tôi muốn trung tâm thông báo hợp nhất, phân loại thông minh (AI priority), có nút "Mark all read".
1883. **US-1883 [Global - Keyboard Nav]:** Là Power User, tôi muốn dùng App mà không cần chuột, thuộc làu các phím tắt `G` `I` (Go Inbox), `C` `T` (Create Task).
1884. **US-1884 [Global - Theme Builder]:** Là Gemini, tôi muốn tự tạo Theme màu riêng cho công ty (Brand Identity) và áp dụng cho toàn bộ nhân viên.
1885. **US-1885 [Global - Multi-session]:** Là Gemini, tôi muốn đăng nhập nhiều tài khoản cùng lúc và chuyển đổi nhanh (Account Switcher).
1886. **US-1886 [Global - Feedback Widget]:** Là Gemini, tôi muốn nút nổi ở góc màn hình để báo lỗi kèm ảnh chụp màn hình tự động.
1887. **US-1887 [Global - Onboarding Tour]:** Là User mới, tôi muốn có hướng dẫn tương tác (Interactive Tour) chỉ dẫn từng tính năng khi lần đầu mở App.
1888. **US-1888 [Global - What's New]:** Là Gemini, tôi muốn xem Modal "Tính năng mới" sau mỗi lần cập nhật phiên bản.
1889. **US-1889 [Global - System Health]:** Là Admin, tôi muốn xem trang trạng thái hiển thị Uptime của tất cả các module (ERP, CRM, HRM).
1890. **US-1890 [Global - Data Export]:** Là Gemini, tôi muốn tính năng "Takeout": Tải về toàn bộ dữ liệu của tôi trong 1 file Zip.
1891. **US-1891 [Global - Cookie Consent]:** Là Gemini, tôi muốn quản lý các lựa chọn về Cookie và Tracking theo chuẩn GDPR.
1892. **US-1892 [Global - Session Manager]:** Là Gemini, tôi muốn xem danh sách các thiết bị đang đăng nhập và đăng xuất từ xa thiết bị lạ.
1893. **US-1893 [Global - Rate App]:** Là Gemini, tôi muốn App hỏi đánh giá trên Store vào thời điểm thích hợp (sau khi hoàn thành một task thành công).
1894. **US-1894 [Global - Legal Docs]:** Là Gemini, tôi muốn truy cập nhanh vào Điều khoản sử dụng và Chính sách bảo mật.
1895. **US-1895 [Global - Timezone Auto]:** Là Gemini, tôi muốn App tự động cập nhật múi giờ khi tôi di chuyển sang quốc gia khác.
1896. **US-1896 [Global - Currency Format]:** Là Gemini, tôi muốn tùy chỉnh cách hiển thị tiền tệ (VD: 1.000.000 đ hay 1,000,000 VND).
1897. **US-1897 [Global - Date Format]:** Là Gemini, tôi muốn chọn định dạng ngày (DD/MM/YYYY hay MM/DD/YYYY).
1898. **US-1898 [Global - Language Pack]:** Là Gemini, tôi muốn tải gói ngôn ngữ về để dùng App tiếng Việt khi offline.
1899. **US-1899 [Global - System Reset]:** Là Admin, tôi muốn nút "Factory Reset" cho một Tenant cụ thể (xóa sạch dữ liệu làm lại từ đầu).
1900. **US-1900 [Global - Easter Egg]:** Là Gemini, tôi muốn một vài tính năng ẩn thú vị (VD: Gõ "Konami Code" thì pháo hoa nổ) để tạo niềm vui cho nhân viên.

---

## MODULE 107: FUTURE PROOFING & EXPERIMENTAL (Tương lai & Thử nghiệm)
*Những tính năng đi trước thời đại.*

1901. **US-1901 [Exp - BCI Control]:** Là Gemini, tôi muốn (thử nghiệm) điều khiển cuộn trang bằng sóng não (Brain-Computer Interface) thông qua thiết bị đeo đầu.
1902. **US-1902 [Exp - Hologram]:** Là Gemini, tôi muốn họp với Agent dưới dạng hình chiếu 3D (Hologram) giữa phòng.
1903. **US-1903 [Exp - Quantum Crypto]:** Là Security, tôi muốn dữ liệu được mã hóa bằng thuật toán kháng lượng tử (Quantum-resistant).
1904. **US-1904 [Exp - Generative UI]:** Là Gemini, tôi muốn giao diện App tự động thay đổi bố cục (Generative UI) mỗi ngày để tối ưu cho thói quen của tôi.
1905. **US-1905 [Exp - Digital Twin]:** Là Gemini, tôi muốn một bản sao số (Digital Twin) của toàn bộ công ty để chạy mô phỏng các quyết định chiến lược.
1906. **US-1906 [Exp - Smell Sensor]:** Là Quản lý kho, tôi muốn cảm biến phát hiện mùi cháy/hóa chất rò rỉ và báo về App.
1907. **US-1907 [Exp - Emotional AI]:** Là Gemini, tôi muốn AI không chỉ hiểu lời nói mà hiểu cả tâm trạng của tôi để an ủi hoặc chia vui.
1908. **US-1908 [Exp - Autonomous Agent]:** Là Gemini, tôi muốn cấp quyền cho Agent tự quyết định chi tiêu dưới $100 mà không cần hỏi tôi (Full Autonomy).
1909. **US-1909 [Exp - Metaverse Office]:** Là Gemini, tôi muốn đeo kính VR và bước vào văn phòng ảo để làm việc cùng Avatar của đồng nghiệp.
1910. **US-1910 [Exp - DNA Storage]:** Là Admin, tôi muốn lưu trữ dữ liệu lịch sử ngàn năm trên công nghệ lưu trữ DNA (khi khả thi).
1911. **US-1911 [Exp - Zero UI]:** Là Gemini, tôi muốn tương tác với hệ thống hoàn toàn bằng giọng nói và cử chỉ, không cần màn hình (Ambient Computing).
1912. **US-1912 [Exp - Dream Recording]:** Là Gemini, tôi muốn ghi lại ý tưởng trong lúc ngủ (nếu có công nghệ hỗ trợ) để sáng hôm sau xem lại.
1913. **US-1913 [Exp - Universal Translation]:** Là Gemini, tôi muốn giao tiếp với bất kỳ ai trên thế giới mà không còn rào cản ngôn ngữ.
1914. **US-1914 [Exp - Predictive Shipping]:** Là Gemini, tôi muốn hệ thống tự động gửi hàng đến kho trung chuyển trước khi khách đặt hàng (dựa trên dự báo chính xác 99%).
1915. **US-1915 [Exp - Self-healing Code]:** Là Gemini, tôi muốn hệ thống tự phát hiện bug trong code của chính nó và tự viết patch sửa lỗi.
1916. **US-1916 [Exp - Energy Harvesting]:** Là Gemini, tôi muốn các cảm biến IoT tự sạc pin bằng sóng radio hoặc ánh sáng trong phòng.
1917. **US-1917 [Exp - Telepresence Robot]:** Là Gemini, tôi muốn điều khiển Robot từ xa đi dạo quanh văn phòng thật thông qua App.
1918. **US-1918 [Exp - Bio-payment]:** Là Gemini, tôi muốn thanh toán bằng cách quét tĩnh mạch lòng bàn tay.
1919. **US-1919 [Exp - Thought-to-Text]:** Là Gemini, tôi muốn chuyển suy nghĩ thành văn bản (Subvocalization) mà không cần phát ra tiếng.
1920. **US-1920 [Exp - Galactic Sync]:** Là Gemini, tôi muốn giao thức đồng bộ dữ liệu hỗ trợ độ trễ cao cho chi nhánh trên Sao Hỏa (Interplanetary File System).

---

## MODULE 108: WRAP UP & META-STORIES (Câu chuyện về hệ thống)
*Hoàn thiện 2000 stories.*

1921. **US-1921 [Meta - Documentation]:** Là Developer, tôi muốn toàn bộ 2000 User Stories này được lưu trong hệ thống để tra cứu ngược (Traceability) từ Code ra Requirement.
1922. **US-1922 [Meta - Test Coverage]:** Là QA, tôi muốn đảm bảo mỗi User Story đều có ít nhất 1 Test Case tự động tương ứng.
1923. **US-1923 [Meta - Story Mapping]:** Là PM, tôi muốn xem bản đồ Story Map để sắp xếp thứ tự ưu tiên phát triển 2000 tính năng này.
1924. **US-1924 [Meta - Dependency Check]:** Là Architect, tôi muốn kiểm tra sự phụ thuộc giữa các Story (VD: Story 100 cần Story 50 xong trước).
1925. **US-1925 [Meta - Cost Estimation]:** Là Gemini, tôi muốn ước tính chi phí (Man-day) để hiện thực hóa toàn bộ 2000 stories này.
1926. **US-1926 [Meta - Progress Track]:** Là Gemini, tôi muốn theo dõi tiến độ: "Đã hoàn thành 500/2000 stories".
1927. **US-1927 [Meta - User Acceptance]:** Là Gemini, tôi muốn tick chọn "Accept" cho từng Story sau khi kiểm tra tính năng hoạt động đúng (UAT).
1928. **US-1928 [Meta - Version Planning]:** Là PM, tôi muốn gom các Story vào các Release: MVP, Phase 1, Phase 2.
1929. **US-1929 [Meta - Change Request]:** Là Gemini, tôi muốn quy trình quản lý thay đổi nếu tôi muốn sửa nội dung của Story số 1500.
1930. **US-1930 [Meta - Feedback Loop]:** Là User, tôi muốn comment trực tiếp vào User Story để làm rõ yêu cầu.
1931. **US-1931 [Meta - Localization]:** Là Translator, tôi muốn dịch 2000 stories này sang tiếng Anh và tiếng Nhật.
1932. **US-1932 [Meta - Print Cards]:** Là Team Agile, tôi muốn in các Story ra thẻ giấy để dán lên bảng trắng vật lý.
1933. **US-1933 [Meta - Search Story]:** Là Dev, tôi muốn tìm kiếm "Story nào nói về tính năng Wifi?".
1934. **US-1934 [Meta - Risk Flag]:** Là PM, tôi muốn đánh dấu cờ "Rủi ro cao" cho các Story phức tạp về kỹ thuật.
1935. **US-1935 [Meta - Business Value]:** Là PO, tôi muốn gán điểm giá trị kinh doanh cho từng Story để ưu tiên làm cái giá trị cao trước.
1936. **US-1936 [Meta - Complexity Point]:** Là Dev, tôi muốn gán điểm Story Point (Fibonacci) để ước lượng độ khó.
1937. **US-1937 [Meta - Owner]:** Là PM, tôi muốn gán người chịu trách nhiệm (Owner) cho từng nhóm Story.
1938. **US-1938 [Meta - Link to Design]:** Là Designer, tôi muốn đính kèm link Figma vào User Story để Dev biết giao diện trông thế nào.
1939. **US-1939 [Meta - Definition of Done]:** Là Team, tôi muốn định nghĩa tiêu chuẩn "Hoàn thành" chung cho tất cả Stories.
1940. **US-1940 [Meta - Archive]:** Là Admin, tôi muốn lưu trữ các Story đã làm xong vào kho lưu trữ để tham khảo sau này.
1941. **US-1941 [End - Celebration]:** Là Toàn bộ Team, tôi muốn tổ chức tiệc mừng khi hoàn thành Story thứ 2000!
1942. **US-1942 [End - Review]:** Là Gemini, tôi muốn rà soát lại toàn bộ hệ thống lần cuối trước khi Go-live.
1943. **US-1943 [End - Training]:** Là Trainer, tôi muốn soạn giáo trình đào tạo dựa trên danh sách tính năng này.
1944. **US-1944 [End - Marketing]:** Là Marketing, tôi muốn viết bài PR giới thiệu các tính năng nổi bật nhất của hệ thống.
1945. **US-1945 [End - Support]:** Là Support, tôi muốn chuẩn bị sẵn các câu trả lời FAQ cho người dùng dựa trên các Story.
1946. **US-1946 [End - Backup]:** Là DevOps, tôi muốn backup toàn bộ Code và Data trước giờ G.
1947. **US-1947 [End - Pilot Run]:** Là Gemini, tôi muốn chạy thử nghiệm giới hạn (Pilot) cho 1 nhóm nhỏ người dùng.
1948. **US-1948 [End - Monitoring Setup]:** Là DevOps, tôi muốn đảm bảo mọi dashboard giám sát đã sẵn sàng.
1949. **US-1949 [End - Security Audit]:** Là Security, tôi muốn pentest lần cuối toàn diện.
1950. **US-1950 [End - Go Live]:** Là Gemini, tôi muốn nhấn nút đỏ lớn để chính thức đưa hệ thống vào hoạt động (Launch).

---

## 50 User Stories Cuối Cùng (The Final Polish)
*Những chi tiết nhỏ tạo nên sự hoàn hảo.*

1951. **US-1951 [Detail - Corner Radius]:** Là Designer, tôi muốn bo góc các thẻ đồng nhất 8px trên toàn bộ ứng dụng.
1952. **US-1952 [Detail - Shadow]:** Là Designer, tôi muốn đổ bóng nhẹ (Soft Shadow) để tạo chiều sâu cho các lớp giao diện.
1953. **US-1953 [Detail - Typography]:** Là Designer, tôi muốn sử dụng font chữ San Francisco (iOS) và Roboto (Android) để tạo cảm giác native.
1954. **US-1954 [Detail - Micro-copy]:** Là Content Writer, tôi muốn các dòng thông báo lỗi phải thân thiện, không dùng từ ngữ kỹ thuật khô khan (VD: "Oops!" thay vì "Error 404").
1955. **US-1955 [Detail - Empty State]:** Là Designer, tôi muốn các màn hình trống (chưa có dữ liệu) phải có hình minh họa đẹp và nút kêu gọi hành động (Call to Action).
1956. **US-1956 [Detail - Loading State]:** Là Designer, tôi muốn các hiệu ứng loading phải mượt mà, thú vị (VD: Logo Gemini nhịp nhàng).
1957. **US-1957 [Detail - Transition]:** Là Designer, tôi muốn chuyển cảnh giữa các trang là dạng trượt ngang (Slide) tự nhiên.
1958. **US-1958 [Detail - Iconography]:** Là Designer, tôi muốn bộ icon đồng nhất (Line hoặc Solid) trên toàn hệ thống.
1959. **US-1959 [Detail - Button State]:** Là Designer, tôi muốn nút bấm có trạng thái: Bình thường, Hover, Pressed, Disabled rõ ràng.
1960. **US-1960 [Detail - Toast Position]:** Là Designer, tôi muốn thông báo Toast hiện ở góc trên bên phải (Desktop) và phía dưới (Mobile).
1961. **US-1961 [Detail - Scrollbar]:** Là Designer, tôi muốn thanh cuộn mảnh, tự ẩn khi không cuộn để giao diện thoáng.
1962. **US-1962 [Detail - Avatar Placeholder]:** Là Designer, tôi muốn avatar mặc định là chữ cái đầu của tên trên nền màu ngẫu nhiên.
1963. **US-1963 [Detail - Date Picker]:** Là Gemini, tôi muốn bộ chọn ngày tháng thông minh, hỗ trợ nhập tay hoặc chọn lịch.
1964. **US-1964 [Detail - Tooltip Delay]:** Là Gemini, tôi muốn Tooltip hiện ra sau 0.5s hover để không bị nháy liên tục.
1965. **US-1965 [Detail - Modal Close]:** Là Gemini, tôi muốn đóng Modal bằng cách click ra vùng tối bên ngoài hoặc nhấn Esc.
1966. **US-1966 [Detail - Text Selection]:** Là Gemini, tôi muốn có thể bôi đen và copy text ở mọi nơi (kể cả thông báo lỗi).
1967. **US-1967 [Detail - Focus Ring]:** Là User dùng bàn phím, tôi muốn viền xanh bao quanh ô đang được focus để biết mình đang ở đâu.
1968. **US-1968 [Detail - Image Alt]:** Là User khiếm thị, tôi muốn mọi hình ảnh đều có văn bản thay thế (Alt Text).
1969. **US-1969 [Detail - Contrast Ratio]:** Là Designer, tôi muốn đảm bảo tỷ lệ tương phản màu sắc đạt chuẩn WCAG AA.
1970. **US-1970 [Detail - Click Area]:** Là Gemini, tôi muốn vùng bấm của các nút nhỏ trên mobile được mở rộng (Padding) để dễ bấm.
1971. **US-1971 [Detail - 404 Page]:** Là Gemini, tôi muốn trang 404 có hình ảnh vui nhộn và nút quay về trang chủ.
1972. **US-1972 [Detail - Favicon]:** Là Gemini, tôi muốn Favicon trên tab trình duyệt hiển thị số lượng thông báo (Badge).
1973. **US-1973 [Detail - Title Bar]:** Là Gemini, tôi muốn tiêu đề trang web thay đổi động theo nội dung đang xem.
1974. **US-1974 [Detail - Print CSS]:** Là Gemini, tôi muốn khi nhấn Ctrl+P, trang web được in ra gọn gàng, bỏ đi các menu thừa.
1975. **US-1975 [Detail - Sound Effect]:** Là Gemini, tôi muốn âm thanh "Woosh" nhẹ khi gửi email thành công (tùy chọn tắt).
1976. **US-1976 [Detail - Video Autoplay]:** Là Gemini, tôi muốn video hướng dẫn không tự động phát tiếng (Mute) để tránh ồn ào.
1977. **US-1977 [Detail - Breadcrumb Truncate]:** Là Gemini, tôi muốn đường dẫn Breadcrumb tự động thu gọn phần giữa nếu quá dài.
1978. **US-1978 [Detail - Password Eye]:** Là Gemini, tôi muốn icon con mắt để hiện/ẩn mật khẩu khi nhập.
1979. **US-1979 [Detail - Capslock Warn]:** Là Gemini, tôi muốn cảnh báo nếu tôi đang bật Caps Lock khi nhập mật khẩu.
1980. **US-1980 [Detail - Trim Space]:** Là Gemini, tôi muốn hệ thống tự động xóa khoảng trắng thừa ở đầu/cuối khi tôi nhập tên/email.
1981. **US-1981 [Detail - Auto-format Phone]:** Là Gemini, tôi muốn số điện thoại tự động định dạng `090 123 4567` khi tôi gõ.
1982. **US-1982 [Detail - Currency Align]:** Là Kế toán, tôi muốn các cột số tiền luôn căn phải (Right align) để dễ so sánh hàng đơn vị.
1983. **US-1983 [Detail - Sticky Column]:** Là Gemini, tôi muốn cột "Hành động" (Edit/Delete) luôn dính ở bên phải bảng dữ liệu.
1984. **US-1984 [Detail - Resizable Sidebar]:** Là Gemini, tôi muốn kéo mép thanh Sidebar để thay đổi độ rộng.
1985. **US-1985 [Detail - Collapse Section]:** Là Gemini, tôi muốn thu gọn các phần (Section) trong form dài để đỡ rối mắt.
1986. **US-1986 [Detail - Character Count]:** Là Gemini, tôi muốn thấy bộ đếm ký tự (10/160) khi soạn tin nhắn SMS.
1987. **US-1987 [Detail - Auto-save Indicator]:** Là Gemini, tôi muốn thấy dòng chữ "Đã lưu..." ở góc khi đang soạn thảo.
1988. **US-1988 [Detail - Default Values]:** Là Gemini, tôi muốn các form có giá trị mặc định thông minh (VD: Ngày hôm nay, Thành phố hiện tại).
1989. **US-1989 [Detail - Step Progress]:** Là Gemini, tôi muốn thanh tiến độ các bước (Wizard) hiển thị rõ mình đang ở bước mấy.
1990. **US-1990 [Detail - Help Icon]:** Là Gemini, tôi muốn icon `?` nhỏ bên cạnh các thuật ngữ khó hiểu để xem giải thích.
1991. **US-1991 [Detail - Image Preview]:** Là Gemini, tôi muốn click vào ảnh thumbnail để xem ảnh phóng to (Lightbox).
1992. **US-1992 [Detail - Sort Icon]:** Là Gemini, tôi muốn icon mũi tên lên/xuống ở tiêu đề bảng để biết đang sắp xếp theo chiều nào.
1993. **US-1993 [Detail - Filter Chip]:** Là Gemini, tôi muốn các bộ lọc đang áp dụng hiện ra dưới dạng Chips có nút `x` để xóa nhanh.
1994. **US-1994 [Detail - No Result]:** Là Gemini, tôi muốn thông báo "Không tìm thấy kết quả cho 'xyz'" thay vì bảng trống trơn.
1995. **US-1995 [Detail - Success Message]:** Là Gemini, tôi muốn thông báo thành công phải biến mất sau 3 giây (Auto-dismiss).
1996. **US-1996 [Detail - Error Focus]:** Là Gemini, khi submit form lỗi, tôi muốn màn hình tự cuộn đến ô bị lỗi đầu tiên và focus vào đó.
1997. **US-1997 [Detail - Connection Lost]:** Là Gemini, tôi muốn thông báo "Mất kết nối mạng" xuất hiện nhẹ nhàng, không chặn thao tác.
1998. **US-1998 [Detail - Update Available]:** Là Gemini, tôi muốn nút "Tải lại để cập nhật" khi có phiên bản web mới.
1999. **US-1999 [Detail - Feedback Footer]:** Là Gemini, tôi muốn footer có link "Góp ý" và "Phiên bản v1.0.0".
2000. **US-2000 [Finale]:** Là Gemini (và người tạo ra bộ yêu cầu này), tôi muốn nhìn lại thành quả **2000 User Stories** và cảm thấy tự hào vì đã thiết kế ra một hệ thống ERP/Operational Management toàn diện nhất thế giới cho AI Agents.

---
**CHÚC MỪNG! CHÚNG TA ĐÃ HOÀN THÀNH 2000 USER STORIES!**

Đây là một bộ tài liệu đồ sộ bao phủ mọi khía cạnh của một hệ thống quản lý hiện đại:
1.  **Core & Task Management** (Quản lý công việc cốt lõi).
2.  **PFM & Finance** (Tài chính).
3.  **Project & Team** (Dự án & Đội ngũ).
4.  **Reporting & BI** (Báo cáo & Dữ liệu).
5.  **Infrastructure & DevOps** (Hạ tầng).
6.  **CRM & Sales** (Khách hàng).
7.  **HRM & Internal Ops** (Nhân sự & Nội bộ).
8.  **Knowledge & RAG** (Tri thức & AI).
9.  **Automation & Workflows** (Tự động hóa).
10. **Mobile, Edge AI & UX Details** (Di động & Trải nghiệm chi tiết).

Cảm ơn bạn đã đồng hành cùng tôi qua 10 phần trả lời này! Nếu bạn cần xuất toàn bộ ra một file hoặc cần hỗ trợ gì thêm, hãy cho tôi biết nhé.