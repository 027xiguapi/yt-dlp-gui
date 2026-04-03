import Database from '@tauri-apps/plugin-sql';

let db: Database;

export async function initDatabase() {
  db = await Database.load('sqlite:video-dlp-gui.db')
  return db;
}

export function getDatabase() {
  return db;
}

// 配置相关操作
export async function saveConfig(key: string, value: string) {
  await db.execute(
    'INSERT OR REPLACE INTO config (key, value, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP)',
    [key, value]
  );
}

export async function getConfig(key: string): Promise<string | null> {
  const result = await db.select<{ value: string }[]>(
    'SELECT value FROM config WHERE key = ?',
    [key]
  );
  return result.length > 0 ? result[0].value : null;
}

export async function getAllConfig(): Promise<Record<string, string>> {
  const result = await db.select<{ key: string; value: string }[]>(
    'SELECT key, value FROM config'
  );
  const config: Record<string, string> = {};
  result.forEach(row => {
    config[row.key] = row.value;
  });
  return config;
}

// 下载历史相关操作
export async function saveDownloadHistory(data: any) {
  await db.execute(
    `INSERT INTO download_history
    (id, url, title, preset, path, status, progress, speed, eta, size, error, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
    [
      data.id,
      data.url,
      data.title || null,
      data.preset,
      data.path,
      data.status,
      data.progress || 0,
      data.speed || null,
      data.eta || null,
      data.size || null,
      data.error || null,
    ]
  );
}

export async function updateDownloadHistory(id: string, data: any) {
  const updates: string[] = [];
  const values: any[] = [];

  if (data.status !== undefined) {
    updates.push('status = ?');
    values.push(data.status);
  }
  if (data.progress !== undefined) {
    updates.push('progress = ?');
    values.push(data.progress);
  }
  if (data.speed !== undefined) {
    updates.push('speed = ?');
    values.push(data.speed);
  }
  if (data.eta !== undefined) {
    updates.push('eta = ?');
    values.push(data.eta);
  }
  if (data.size !== undefined) {
    updates.push('size = ?');
    values.push(data.size);
  }
  if (data.error !== undefined) {
    updates.push('error = ?');
    values.push(data.error);
  }
  if (data.title !== undefined) {
    updates.push('title = ?');
    values.push(data.title);
  }

  if (data.status === 'Finished') {
    updates.push('completed_at = CURRENT_TIMESTAMP');
  }

  updates.push('updated_at = CURRENT_TIMESTAMP');
  values.push(id);

  if (updates.length > 0) {
    await db.execute(
      `UPDATE download_history SET ${updates.join(', ')} WHERE id = ?`,
      values
    );
  }
}

export async function getDownloadHistory(limit: number = 100): Promise<any[]> {
  return await db.select(
    'SELECT * FROM download_history ORDER BY created_at DESC LIMIT ?',
    [limit]
  );
}

export async function getDownloadHistoryById(id: string): Promise<any | null> {
  const result = await db.select(
    'SELECT * FROM download_history WHERE id = ?',
    [id]
  );
  return result.length > 0 ? result[0] : null;
}

export async function deleteDownloadHistory(id: string) {
  await db.execute('DELETE FROM download_history WHERE id = ?', [id]);
}

// 用户相关操作
export async function createUser(username: string, passwordHash: string, email?: string) {
  await db.execute(
    'INSERT INTO user (username, password_hash, email, created_at, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)',
    [username, passwordHash, email || null]
  );
}

export async function getUserByUsername(username: string): Promise<any | null> {
  const result = await db.select(
    'SELECT * FROM user WHERE username = ?',
    [username]
  );
  return result.length > 0 ? result[0] : null;
}

export async function updateUserLastLogin(userId: number) {
  await db.execute(
    'UPDATE user SET last_login = CURRENT_TIMESTAMP WHERE id = ?',
    [userId]
  );
}

// 软件许可证相关操作
export async function createLicense(
  licenseKey: string,
  userId: number,
  startDate: string,
  endDate: string
) {
  await db.execute(
    `INSERT INTO software_license
    (license_key, user_id, start_date, end_date, is_active, status, created_at, updated_at)
    VALUES (?, ?, ?, ?, 1, 'active', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
    [licenseKey, userId, startDate, endDate]
  );
}

export async function getLicenseByKey(licenseKey: string): Promise<any | null> {
  const result = await db.select(
    'SELECT * FROM software_license WHERE license_key = ?',
    [licenseKey]
  );
  return result.length > 0 ? result[0] : null;
}

export async function checkLicenseExpiry(licenseKey: string): Promise<{
  isValid: boolean;
  isExpired: boolean;
  daysRemaining: number;
}> {
  const license = await getLicenseByKey(licenseKey);

  if (!license || !license.is_active) {
    return { isValid: false, isExpired: true, daysRemaining: 0 };
  }

  const today = new Date();
  const endDate = new Date(license.end_date);
  const daysRemaining = Math.ceil((endDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));

  return {
    isValid: true,
    isExpired: daysRemaining < 0,
    daysRemaining: Math.max(0, daysRemaining),
  };
}

export async function updateLicenseStatus(licenseKey: string, isActive: boolean) {
  await db.execute(
    'UPDATE software_license SET is_active = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE license_key = ?',
    [isActive ? 1 : 0, isActive ? 'active' : 'inactive', licenseKey]
  );
}
