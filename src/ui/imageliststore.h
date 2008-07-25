/*
 * niepce - ui/imageliststore.h
 *
 * Copyright (C) 2008 Hubert Figuiere
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */



#ifndef __UI_IMAGELISTSTORE__
#define __UI_IMAGELISTSTORE__


#include <gdkmm/pixbuf.h>
#include <gtkmm/liststore.h>

#include "framework/notification.h"
#include "framework/controller.h"
#include "db/libfile.h"
#include "libraryclient/libraryclient.h"

namespace ui {

/** @brief the general list store */
class ImageListStore 
    : public Gtk::ListStore
{
public:
    class Columns 
        : public Gtk::TreeModelColumnRecord
    {
    public:
        enum {
            THUMB_INDEX = 0,
            FILE_INDEX = 1,
            STRIP_THUMB_INDEX = 2
        };
        Columns()
            { 
                add(m_pix);
                add(m_libfile);
                add(m_strip_thumb);
            }
        Gtk::TreeModelColumn<Glib::RefPtr<Gdk::Pixbuf> > m_pix;
        Gtk::TreeModelColumn<db::LibFile::Ptr> m_libfile;
        Gtk::TreeModelColumn<Glib::RefPtr<Gdk::Pixbuf> > m_strip_thumb;
    };

    const Columns & columns() const
        { return m_columns; }
    Gtk::TreePath get_path_from_id(int id);
    Gtk::TreeIter get_iter_from_id(int id);

    static Glib::RefPtr<ImageListStore> create();

    void set_parent_controller(const framework::Controller::WeakPtr & ctrl)
        { m_controller = ctrl; }

    void on_lib_notification(const framework::Notification::Ptr &n);
    void on_tnail_notification(const framework::Notification::Ptr &n);
protected:
    ImageListStore(const Columns& columns);
private:
    libraryclient::LibraryClient::Ptr getLibraryClient();

    const Columns           & m_columns;
    std::map<int, Gtk::TreeIter> m_idmap;
    framework::Controller::WeakPtr m_controller;
};

}

/*
  Local Variables:
  mode:c++
  c-file-style:"stroustrup"
  c-file-offsets:((innamespace . 0))
  indent-tabs-mode:nil
  fill-column:99
  End:
*/

#endif