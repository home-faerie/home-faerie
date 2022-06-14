Home Faerie
===========

Home Faerie is a set of lightweight utilities ([auto]magical faeries)
for various home monitoring, data logging and automation tasks.

Why?
----

Because I wanted something simple and lightweight :)

It started with a set of Python scripts running on a Olimex Lime 2:

* zigbee2mqtt data logger to PostgreSQL
* simple utility script to for a zigbee button which then toggles multiple
  tasmota-controlled lights.

And now Rust - I can just drop a single binary to filesystem and restart
the systemd service.

Features (so far..)
-------------------

* Zigbee2MQTT sensor data logging to PostgreSQL database

Setup
-----

Create database schema:

.. code-block:: sql

    CREATE TABLE device_meters (
        "timestamp" timestamp with time zone,
        name text,
        value double precision,
        dimensions jsonb,
        value_meta json
    );

    CREATE INDEX ON device_meters (name, timestamp);
    CREATE INDEX ON device_meters USING GIN (dimensions);


And run the application:

.. code-block:: text

    $ POSTGRESQL_URL=postgresql:/meters MQTT_URI=mqtt://zigbee2mqtt-gateway.locaal?client_id=home-faerie-123 cargo run

Visualizations
--------------

Panels in Grafana:

.. code-block:: sql

    SELECT
      timestamp as time,
      dimensions->>'id' AS device,
      value
    FROM
      device_meters
    WHERE
      $__timeFilter("timestamp")
      AND name = 'temperature'
    ORDER BY 1,2
