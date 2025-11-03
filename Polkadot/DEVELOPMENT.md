# Hướng Dẫn Phát Triển Blog Pallet

## Mục Lục
1. [Build và Test](#1-build-và-test)
2. [Chạy Node](#2-chạy-node)
3. [Test Các Tính Năng](#3-test-các-tính-năng)
4. [Sử dụng Polkadot.js Apps](#4-sử-dụng-polkadotjs-apps)
5. [Viết Frontend](#5-viết-frontend)

---

## 1. Build và Test

### 1.1. Test Pallet
Chạy unit tests cho pallet blog:

```bash
cd Polkadot
cargo test -p blog-pallet
```

Kiểm tra một test cụ thể:
```bash
cargo test -p blog-pallet test::create_post_works
```

### 1.2. Build Runtime
Build runtime với pallet blog đã tích hợp:

```bash
# Build runtime (debug mode - nhanh hơn)
cargo build -p dacs-runtime

# Build runtime (release mode - tối ưu)
cargo build -p dacs-runtime --release
```

### 1.3. Build Node
Build node để chạy blockchain:

```bash
# Build node (debug mode)
cargo build

# Build node (release mode - khuyến nghị)
cargo build --release
```

Binary sẽ được tạo tại: `target/release/dacs-node`

---

## 2. Chạy Node

### 2.1. Chạy Node Development Mode (Standalone)
Chạy node độc lập (không cần relay chain) để test nhanh:

```bash
# Chạy với default chain spec
./target/release/dacs-node --dev --tmp

# Hoặc với các options khác
./target/release/dacs-node \
  --dev \
  --tmp \
  --rpc-external \
  --ws-external \
  --rpc-cors all
```

**Lưu ý:** `--dev` mode sẽ tự động tạo một số accounts có sẵn:
- Alice (key: `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`)
- Bob (key: `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`)
- Charlie, Dave, Eve, Ferdie

### 2.2. Chạy với Relay Chain (Parachain Mode)
Nếu cần chạy như parachain thật:

```bash
# Terminal 1: Chạy relay chain (Polkadot hoặc Rococo)
# Download Polkadot binary hoặc build từ source

# Terminal 2: Chạy parachain node
./target/release/dacs-node \
  --collator \
  --tmp \
  --chain local-testnet \
  -- \
  --chain rococo-local \
  --execution wasm
```

### 2.3. Các Flags Quan Trọng
- `--dev`: Development mode, tự động tạo accounts
- `--tmp`: Sử dụng temporary database (xóa khi tắt node)
- `--rpc-external`: Cho phép RPC từ bên ngoài
- `--ws-external`: Cho phép WebSocket từ bên ngoài
- `--rpc-cors all`: Cho phép CORS (cần cho frontend)
- `--alice`: Chạy với Alice account (collator mode)

---

## 3. Test Các Tính Năng

### 3.1. Sử dụng Polkadot.js CLI
Cài đặt Polkadot.js CLI:
```bash
npm install -g @polkadot/api-cli
```

Kết nối và test:
```bash
# Kết nối đến node
polkadot-js-api --ws ws://127.0.0.1:9944

# Xem metadata
polkadot-js-api --ws ws://127.0.0.1:9944 --info
```

### 3.2. Sử dụng Substrate API Sidebar
Khi node đang chạy, bạn có thể sử dụng:
- **RPC endpoint**: `http://127.0.0.1:9933`
- **WebSocket endpoint**: `ws://127.0.0.1:9944`

---

## 4. Sử dụng Polkadot.js Apps

### 4.1. Kết nối đến Local Node
1. Mở trình duyệt và truy cập: https://polkadot.js.org/apps
2. Ở góc trên bên trái, click vào dropdown network
3. Chọn "Development" → "Local Node"
4. Hoặc thủ công: Settings → Endpoint → Custom → nhập `ws://127.0.0.1:9944`

### 4.2. Test Các Extrinsics

#### Tạo Bài Viết
1. Navigate đến: **Developer** → **Extrinsics**
2. Chọn account: **Alice**
3. Chọn pallet: **blog**
4. Chọn method: **createPost**
5. Nhập tham số:
   - `title`: `My First Post`
   - `content`: `This is my first blog post on blockchain!`
6. Click **Submit Transaction**
7. Xác nhận transaction

#### Like Bài Viết
1. Pallet: **blog**
2. Method: **togglePostLike**
3. `post_id`: `0` (ID của bài viết vừa tạo)
4. Submit

#### Thêm Tags
1. Pallet: **blog**
2. Method: **addTags**
3. `post_id`: `0`
4. `tags`: `[["rust"], ["blockchain"]]` (array of byte arrays)
5. Submit

#### Bookmark Bài Viết
1. Pallet: **blog**
2. Method: **toggleBookmark**
3. `post_id`: `0`
4. Submit

#### Follow Tác Giả
1. Pallet: **blog**
2. Method: **toggleFollow**
3. `author`: `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` (Alice's address)
4. Submit (dùng Bob account để follow Alice)

### 4.3. Xem Storage/State
1. Navigate đến: **Developer** → **Chain State**
2. Chọn query: **blog** pallet
3. Xem các storage items:
   - `posts(postId)`: Xem thông tin bài viết
   - `postLikes(postId)`: Xem số lượt like
   - `postTags(postId)`: Xem tags của bài viết
   - `authorFollowers(author)`: Xem số followers
   - `userBookmarks(user)`: Xem bookmarks của user

### 4.4. Xem Events
1. Navigate đến: **Network** → **Explorer**
2. Xem các events được emit:
   - `blog.PostCreated`
   - `blog.PostLiked`
   - `blog.PostTagged`
   - etc.

---

## 5. Viết Frontend

### 5.1. Setup Project với Polkadot.js API

Tạo project mới:
```bash
npm init -y
npm install @polkadot/api @polkadot/extension-dapp
```

### 5.2. Kết nối đến Node
```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });

// Xem metadata
console.log('Runtime version:', api.runtimeVersion);
```

### 5.3. Tạo Bài Viết
```javascript
// Kết nối wallet extension
const { web3Accounts, web3Enable, web3FromAddress } = require('@polkadot/extension-dapp');

// Enable extension
await web3Enable('blog-dapp');

// Get accounts
const accounts = await web3Accounts();
const account = accounts[0];

// Get signer
const injector = await web3FromAddress(account.address);

// Tạo bài viết
const tx = api.tx.blog.createPost(
  'My Post Title',
  'My Post Content'
);

// Sign và submit
await tx.signAndSend(account.address, { signer: injector.signer }, ({ status, events }) => {
  if (status.isInBlock) {
    console.log('Transaction in block:', status.asInBlock.toString());
  }
});
```

### 5.4. Đọc Storage
```javascript
// Đọc thông tin bài viết
const post = await api.query.blog.posts(0);
console.log('Post:', post.toHuman());

// Đọc số lượt like
const likes = await api.query.blog.postLikes(0);
console.log('Likes:', likes.toNumber());

// Đọc tags
const tags = await api.query.blog.postTags(0);
console.log('Tags:', tags.toHuman());
```

### 5.5. Subscribe Events
```javascript
// Subscribe events
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;
    
    if (event.section === 'blog') {
      console.log('Blog event:', event.method, event.data.toHuman());
    }
  });
});
```

---

## 6. Debugging và Troubleshooting

### 6.1. Xem Logs
Khi chạy node, bạn sẽ thấy logs. Để xem logs chi tiết hơn:

```bash
RUST_LOG=debug ./target/release/dacs-node --dev --tmp
```

### 6.2. Kiểm tra Errors
Nếu transaction fail:
1. Xem error trong Polkadot.js Apps (Network → Explorer)
2. Kiểm tra logs của node
3. Xem storage để đảm bảo state đúng

### 6.3. Reset Chain
Nếu cần reset hoàn toàn:
```bash
# Xóa database
rm -rf /tmp/.local/share/dacs-node

# Hoặc nếu dùng --tmp, chỉ cần tắt và chạy lại với --tmp
```

---

## 7. Các Lệnh Hữu Ích

### Build Commands
```bash
# Build tất cả
cargo build --release

# Build chỉ runtime
cargo build -p dacs-runtime --release

# Build chỉ node
cargo build -p dacs-node --release

# Test tất cả
cargo test

# Test chỉ pallet
cargo test -p blog-pallet
```

### Node Commands
```bash
# Xem help
./target/release/dacs-node --help

# Xem thông tin chain
./target/release/dacs-node --chain-info

# Export genesis state
./target/release/dacs-node export-genesis-state --chain dev > genesis-state
```

---

## 8. Tiếp Theo

### 8.1. Cải thiện Pallet
- Thêm pagination cho queries
- Thêm search functionality (có thể dùng off-chain workers)
- Thêm moderation features
- Thêm reputation system

### 8.2. Frontend Development
- Tạo React/Vue app
- Integrate với Polkadot.js extension
- Tạo UI/UX đẹp cho blog
- Thêm image support (có thể dùng IPFS)

### 8.3. Testing
- Viết integration tests
- Viết E2E tests với frontend
- Load testing
- Security audit

### 8.4. Documentation
- Viết API documentation
- Tạo user guide
- Viết developer guide
- Tạo video tutorials

---

## 9. Resources

- [Polkadot.js Documentation](https://polkadot.js.org/docs/)
- [Substrate Documentation](https://docs.substrate.io/)
- [FRAME Pallet Development](https://docs.substrate.io/tutorials/build-application-logic/)
- [Polkadot.js Apps](https://polkadot.js.org/apps)

---

Chúc bạn phát triển thành công! 🚀

