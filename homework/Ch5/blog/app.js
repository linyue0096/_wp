const express = require('express');
const session = require('express-session');
const bcrypt = require('bcryptjs');
const db = require('./database');

const app = express();
const PORT = 3000;

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(express.static('public'));
app.use(session({
  secret: 'threads_secret_key',
  resave: false,
  saveUninitialized: false,
  cookie: { maxAge: 7 * 24 * 60 * 60 * 1000 }
}));

function isAuthenticated(req, res, next) {
  if (req.session.userId) return next();
  res.status(401).json({ error: '請先登入' });
}

app.get('/', (req, res) => {
  res.sendFile(__dirname + '/public/index.html');
});

function getUserWithStats(userId) {
  return new Promise((resolve, reject) => {
    const sql = `
      SELECT u.*,
        (SELECT COUNT(*) FROM follows WHERE following_id = u.id) as followers_count,
        (SELECT COUNT(*) FROM follows WHERE follower_id = u.id) as following_count,
        (SELECT COUNT(*) FROM posts WHERE user_id = u.id) as posts_count
      FROM users u WHERE u.id = ?
    `;
    db.get(sql, [userId], (err, user) => {
      if (err) reject(err);
      else resolve(user);
    });
  });
}

app.post('/register', (req, res) => {
  const { username, password, display_name } = req.body;
  if (!username || !password) {
    return res.status(400).json({ error: '請填寫帳號和密碼' });
  }

  const hash = bcrypt.hashSync(password, 10);
  const displayName = display_name || username;

  db.run(
    'INSERT INTO users (username, password, display_name) VALUES (?, ?, ?)',
    [username, hash, displayName],
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
      return res.json({ message: '登入成功', user: { id: user.id, username: user.username, display_name: user.display_name } });
    }

    res.status(401).json({ error: '帳號或密碼錯誤' });
  });
});

app.post('/logout', (req, res) => {
  req.session.destroy();
  res.json({ message: '已登出' });
});

app.get('/me', isAuthenticated, (req, res) => {
  getUserWithStats(req.session.userId).then(user => {
    delete user.password;
    res.json(user);
  });
});

app.put('/me', isAuthenticated, (req, res) => {
  const { display_name, bio, avatar } = req.body;
  db.run(
    'UPDATE users SET display_name = COALESCE(?, display_name), bio = COALESCE(?, bio), avatar = COALESCE(?, avatar) WHERE id = ?',
    [display_name, bio, avatar, req.session.userId],
    function (err) {
      if (err) return res.status(500).json({ error: '更新失敗' });
      res.json({ message: '更新成功' });
    }
  );
});

app.get('/users/:id', (req, res) => {
  const userId = req.params.id;
  getUserWithStats(userId).then(user => {
    if (!user) return res.status(404).json({ error: '用戶不存在' });
    delete user.password;
    
    if (req.session.userId) {
      db.get('SELECT 1 FROM follows WHERE follower_id = ? AND following_id = ?', [req.session.userId, userId], (err, follow) => {
        user.is_following = !!follow;
        res.json(user);
      });
    } else {
      user.is_following = false;
      res.json(user);
    }
  });
});

app.post('/follow/:id', isAuthenticated, (req, res) => {
  const targetId = req.params.id;
  if (targetId == req.session.userId) {
    return res.status(400).json({ error: '無法追蹤自己' });
  }

  db.run(
    'INSERT OR IGNORE INTO follows (follower_id, following_id) VALUES (?, ?)',
    [req.session.userId, targetId],
    function (err) {
      if (err) return res.status(500).json({ error: '追蹤失敗' });
      res.json({ message: '已追蹤' });
    }
  );
});

app.delete('/follow/:id', isAuthenticated, (req, res) => {
  const targetId = req.params.id;
  db.run(
    'DELETE FROM follows WHERE follower_id = ? AND following_id = ?',
    [req.session.userId, targetId],
    function (err) {
      if (err) return res.status(500).json({ error: '取消追蹤失敗' });
      res.json({ message: '已取消追蹤' });
    }
  );
});

app.get('/feed', (req, res) => {
  const feedSql = `
    SELECT p.*, u.username, u.display_name, u.avatar,
      (SELECT COUNT(*) FROM likes WHERE post_id = p.id) as likes_count,
      EXISTS(SELECT 1 FROM likes WHERE post_id = p.id AND user_id = ?) as liked
    FROM posts p
    JOIN users u ON p.user_id = u.id
    WHERE p.user_id IN (
      SELECT following_id FROM follows WHERE follower_id = ?
    ) OR p.user_id = ?
    ORDER BY p.created_at DESC
  `;
  
  const userId = req.session.userId || null;
  db.all(feedSql, [userId, userId, userId], (err, posts) => {
    if (err) return res.status(500).json({ error: '讀取失敗' });
    res.json(posts);
  });
});

app.get('/posts', (req, res) => {
  const sql = `
    SELECT p.*, u.username, u.display_name, u.avatar,
      (SELECT COUNT(*) FROM likes WHERE post_id = p.id) as likes_count,
      EXISTS(SELECT 1 FROM likes WHERE post_id = p.id AND user_id = ?) as liked
    FROM posts p
    JOIN users u ON p.user_id = u.id
    ORDER BY p.created_at DESC
  `;
  
  const userId = req.session.userId || null;
  db.all(sql, [userId], (err, posts) => {
    if (err) return res.status(500).json({ error: '讀取失敗' });
    res.json(posts);
  });
});

app.get('/users/:id/posts', (req, res) => {
  const userId = req.params.id;
  const sql = `
    SELECT p.*, u.username, u.display_name, u.avatar,
      (SELECT COUNT(*) FROM likes WHERE post_id = p.id) as likes_count,
      EXISTS(SELECT 1 FROM likes WHERE post_id = p.id AND user_id = ?) as liked
    FROM posts p
    JOIN users u ON p.user_id = u.id
    WHERE p.user_id = ?
    ORDER BY p.created_at DESC
  `;
  
  const currentUserId = req.session.userId || null;
  db.all(sql, [currentUserId, userId], (err, posts) => {
    if (err) return res.status(500).json({ error: '讀取失敗' });
    res.json(posts);
  });
});

app.post('/posts', isAuthenticated, (req, res) => {
  const { content, image_url } = req.body;
  if (!content) {
    return res.status(400).json({ error: '內容為必填' });
  }

  db.run(
    'INSERT INTO posts (user_id, content, image_url) VALUES (?, ?, ?)',
    [req.session.userId, content, image_url || null],
    function (err) {
      if (err) return res.status(500).json({ error: '發布失敗' });
      res.json({ message: '發布成功', id: this.lastID });
    }
  );
});

app.delete('/posts/:id', isAuthenticated, (req, res) => {
  const postId = req.params.id;
  
  db.get('SELECT * FROM posts WHERE id = ?', [postId], (err, post) => {
    if (!post) return res.status(404).json({ error: '貼文不存在' });
    if (post.user_id !== req.session.userId) {
      return res.status(403).json({ error: '無法刪除他人的貼文' });
    }
    
    db.run('DELETE FROM likes WHERE post_id = ?', [postId], () => {
      db.run('DELETE FROM posts WHERE id = ?', [postId], (err) => {
        if (err) return res.status(500).json({ error: '刪除失敗' });
        res.json({ message: '刪除成功' });
      });
    });
  });
});

app.post('/posts/:id/like', isAuthenticated, (req, res) => {
  const postId = req.params.id;
  
  db.run(
    'INSERT OR IGNORE INTO likes (user_id, post_id) VALUES (?, ?)',
    [req.session.userId, postId],
    function (err) {
      if (err) return res.status(500).json({ error: '按讚失敗' });
      res.json({ message: '已按讚' });
    }
  );
});

app.delete('/posts/:id/like', isAuthenticated, (req, res) => {
  const postId = req.params.id;
  
  db.run(
    'DELETE FROM likes WHERE user_id = ? AND post_id = ?',
    [req.session.userId, postId],
    function (err) {
      if (err) return res.status(500).json({ error: '取消按讚失敗' });
      res.json({ message: '已取消按讚' });
    }
  );
});

app.listen(PORT, () => {
  console.log(`Threads 風格網站運行於 http://localhost:${PORT}`);
});

module.exports = app;