#ifndef QPOINT_H
#define QPOINT_H

#include "moqt_core_exports.h"

class MOQT_CORE_EXPORT QPoint {
public:
    QPoint() { m_x = m_y = 0; }
    QPoint(int x, int y) {
        m_x = x;
        m_y = y;
    }
    int x() const { return m_x; }
    int y() const { return m_y; }
    void setX(int x) { m_x = x; }
    void setY(int y) { m_y = y; }

    QPoint operator+(const QPoint& other);
private:
    int m_x, m_y;

    friend QPoint operator-(const QPoint& one, const QPoint& other);
};

MOQT_CORE_EXPORT QPoint operator-(const QPoint& one, const QPoint& other);

#endif //QPOINT_H
