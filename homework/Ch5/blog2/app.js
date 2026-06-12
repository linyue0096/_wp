const express = require('express');
const session = require('express-session');
const bcrypt = require('bcryptjs');
const db = require('./database');

const app = express();
const PORT = 3000;

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(session({
  secret: 'blog_secret_key',
  resave: false,
  saveUninitialized: false,
  cookie: { maxAge: 24 * 60 * 60 * 1000 }
}));

function isAuthenticated(req, res, next) {
  if (req.session.userId) return next();
  res.status(401).json({ error: '請先登入' });
}

app.get('/', (req, res) => {
  res.send('Blog 2 is running!');
});

app.post('/register', (req, res) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).json({ error: '請填寫帳號和密碼' });
  }

  const hash = bcrypt.hashSync(password, 10);

  db.run(
    'INSERT INTO users (username, password) VALUES (?, ?)',
    [username, hash],
    function (err) {
      if (err) {
        return res.status(400).json({ error: '帳號已存在' });
      }
      res.json({ message: '註冊成功', userId: this.lastID });
    }
  );
});

app.post('/login', (req, res) => {
  const { username, password } = req.body;

  db.get('SELECT * FROM users WHERE username = ?', [username], (err, user) => {
    if (err || !user) {
      return res.status(401).json({ error: '帳號或密碼錯誤' });
    }

    if (bcrypt.compareSync(password, user.password)) {
      req.session.userId = user.id;
      req.session.username = user.username;
      return res.json({ message: '登入成功', username: user.username });
    }

    res.status(401).json({ error: '帳號或密碼錯誤' });
  });
});

app.post('/logout', (req, res) => {
  req.session.destroy();
  res.json({ message: '已登出' });
});

app.get('/posts', (req, res) => {
  db.all('SELECT posts.*, users.username FROM posts LEFT JOIN users ON posts.author_id = users.id ORDER BY created_at DESC', [], (err, posts) => {
    if (err) return res.status(500).json({ error: '讀取失敗' });
    res.json(posts);
  });
});

app.get('/posts/:id', (req, res) => {
  const id = req.params.id;

  db.get('SELECT posts.*, users.username FROM posts LEFT JOIN users ON posts.author_id = users.id WHERE posts.id = ?', [id], (err, post) => {
    if (err || !post) return res.status(404).json({ error: '文章不存在' });

    db.all('SELECT * FROM comments WHERE post_id = ? ORDER BY created_at DESC', [id], (err, comments) => {
      res.json({ post, comments });
    });
  });
});

app.post('/posts', isAuthenticated, (req, res) => {
  const { title, content } = req.body;
  if (!title || !content) {
    return res.status(400).json({ error: '標題和內容為必填' });
  }

  db.run(
    'INSERT INTO posts (title, content, author_id) VALUES (?, ?, ?)',
    [title, content, req.session.userId],
    function (err) {
      if (err) return res.status(500).json({ error: '建立失敗' });
      res.json({ message: '文章建立成功', id: this.lastID });
    }
  );
});

app.put('/posts/:id', isAuthenticated, (req, res) => {
  const { title, content } = req.body;
  const id = req.params.id;

  db.run(
    'UPDATE posts SET title = ?, content = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?',
    [title, content, id],
    function (err) {
      if (err) return res.status(500).json({ error: '更新失敗' });
      res.json({ message: '文章更新成功' });
    }
  );
});

app.delete('/posts/:id', isAuthenticated, (req, res) => {
  const id = req.params.id;

  db.run('DELETE FROM posts WHERE id = ?', [id], function (err) {
    if (err) return res.status(500).json({ error: '刪除失敗' });
    res.json({ message: '文章刪除成功' });
  });
});

app.post('/posts/:id/comments', (req, res) => {
  const { author_name, content } = req.body;
  const post_id = req.params.id;

  if (!author_name || !content) {
    return res.status(400).json({ error: '請填寫名稱和內容' });
  }

  db.run(
    'INSERT INTO comments (post_id, author_name, content) VALUES (?, ?, ?)',
    [post_id, author_name, content],
    function (err) {
      if (err) return res.status(500).json({ error: '留言失敗' });
      res.json({ message: '留言成功' });
    }
  );
});

app.listen(PORT, () => {
  console.log(`伺服器運行於 http://localhost:${PORT}`);
});

module.exports = app;