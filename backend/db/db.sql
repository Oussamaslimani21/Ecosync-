-- @block
CREATE TABLE devices(
  id VARCHAR(255),
  position VARCHAR(255),
  address VARCHAR(255),
  is_vehicle BOOLEAN,
  PRIMARY KEY(id)
);

-- @block
CREATE TABLE sensors(
  id INT AUTO_INCREMENT,
  name VARCHAR(255),
  unit VARCHAR(255),
  PRIMARY KEY(id)
);

-- @block
CREATE TABLE data(
  id BIGINT AUTO_INCREMENT,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
  values VARCHAR(255),
  device_uuid VARCHAR(255),
  sensor INT,
  PRIMARY KEY(id),
  FOREIGN KEY(device_uuid) REFERENCES devices(id),
  FOREIGN KEY(sensor) REFERENCES sensors(id),
);
